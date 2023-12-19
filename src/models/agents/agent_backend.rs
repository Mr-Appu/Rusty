use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_templates_content, read_exe_main_content, save_api_endpoints,
    save_backend_code, WEB_SERVER_PROJECT_PATH,
};

use crate::helpers::command_line::{confirm_safe_code, PrintCommand};
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_templates_content();

        // Concatenate Code Template and Instruction
        let msg_content: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_content,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_content: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_content,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_content: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_content,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code: String = read_exe_main_content();

        // Message Content
        let msg_content: String = format!("CODE_INPUT: {}", backend_code);

        let ai_response: String = ai_task_request(
            msg_content,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        return ai_response;
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }
                AgentState::UnitTesting => {
                    // ENSURE AI SAFETY
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Requesting user input",
                    );

                    let is_safe_code: bool = confirm_safe_code();

                    if !is_safe_code {
                        panic!("Configure AI / API Endpoints Again !!!");
                    }

                    // BUILD and TEST Code
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Building project ...",
                    );

                    // Build Code
                    let build_backend_code: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to Build Backend Code");

                    // Determine build error
                    if build_backend_code.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Test server build successful...",
                        );
                    } else {
                        let error_vec: Vec<u8> = build_backend_code.stderr;
                        let error_str: String = String::from_utf8(error_vec).unwrap();

                        // Update error stats
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        // Exit if Too Many Bugs
                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in code",
                            );
                            panic!("Error: Too many bugs")
                        }

                        PrintCommand::Issue.print_agent_message(
                            self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Build Failed, Working on it Again !!!",
                        );

                        // Pass back to Fix Bug Code
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    // Extract and Test : Rest API Endpoints

                    // Extract API Endpoints
                    let api_endpoints_str: String = self.call_extract_rest_api_endpoints().await;

                    // Convert API Endpoints into Json Values
                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to Decode API Endpoints");

                    // Filter Endpoints to check
                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    // Store API Endpoints
                    factsheet.api_endpoint_schema = Some(check_endpoints.clone());

                    // Run backend application
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Starting web server...",
                    );

                    // Run Backend Code
                    let mut run_backend_code: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to run Backend Code");

                    // Testing on server will take place soon
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Launching tests on server in 5 seconds...",
                    );

                    let seconds_sleep: Duration = Duration::from_secs(5);
                    time::sleep(seconds_sleep).await;

                    // Run Tests on API Endpoints
                    for endpoint in check_endpoints {
                        // Display Message
                        let test_msg: String = format!("Testing Endpoint '{}'...", endpoint.route);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            test_msg.as_str(),
                        );

                        // Create Client
                        let client: Client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // Test URL
                        let url: String = format!("http://localhost:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_msg: String = format!(
                                        "WARNING: Failed to call backend url endpoint {}",
                                        endpoint.route
                                    );
                                    PrintCommand::Issue.print_agent_message(
                                        self.attributes.position.as_str(),
                                        err_msg.as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                run_backend_code
                                    .kill()
                                    .expect("Failed to kill backend server");
                                let err_msg: String = format!("Error checking backend {}", e);
                                PrintCommand::Issue.print_agent_message(
                                    self.attributes.position.as_str(),
                                    err_msg.as_str(),
                                );
                            }
                        }
                    }

                    save_api_endpoints(&api_endpoints_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend testing complete...",
                    );

                    run_backend_code
                        .kill()
                        .expect("Failed to kill backend server");

                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_backend_code() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"
        {
            "project_description": "need a website that lets me play worlde against the website. The webserver needs to keep track of the users last move ",
            "project_scope": {
              "is_crud_required": false,
              "is_user_login_and_logout": false,
              "is_external_urls_required": false
            },
            "external_urls": [],
            "backend_code": null,
            "api_endpoint_schema": null
        }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent
            .execute(&mut factsheet)
            .await
            .expect("Backend Developer Agent Failed");
    }
}
