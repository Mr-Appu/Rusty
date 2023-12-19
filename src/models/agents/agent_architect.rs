use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve Project Scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_content: String = format!("{}", factsheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_content,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);

        return ai_response;
    }

    // Retrieve Project Scope
    async fn call_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_urls = Some(ai_response.clone());
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Important !!!
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope: ProjectScope = self.call_project_scope(factsheet).await;

                    // Confirm if external URLS
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut include_urls: Vec<String> = vec![];

                    let client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    let urls: &Vec<String> = factsheet
                        .external_urls
                        .as_ref()
                        .expect("No URL object on factsheet");

                    // Exclude Faulty URL
                    for url in urls {
                        let endpoint_str: String = format!("Testing URL Endpoint {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );

                        // Perform URL Test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code == 200 {
                                    include_urls.push(url.clone())
                                }
                            }
                            Err(e) => println!("Error Checking {} : {}", url, e),
                        };
                    }

                    // Update FactSheet External Urls
                    factsheet.external_urls = Some(include_urls);

                    // Confirm done
                    self.attributes.state = AgentState::Finished;
                }
                // Default to Finished state
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_solutions_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut factsheet: FactSheet = FactSheet {
            project_description: "Build me a web application that translates given input into any language of the user's choice".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None
        };

        agent
            .execute(&mut factsheet)
            .await
            .expect("Solution Architect Agent Failed !!!");
        assert!(factsheet.project_scope != None);

        dbg!(factsheet);
    }
}
