use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Text;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

const CODE_TEMPLATE_PATH: &str = r"D:\Rust\web_template\src\code_template.rs"; // Path of the "CODE TEMPLATE" file
pub const WEB_SERVER_PROJECT_PATH: &str = r"D:\Rust\web_template\"; // Path of the "Directory" where Agent's code will be saved.
pub const EXEC_MAIN_PATH: &str = r"D:\Rust\web_template\src\main.rs"; // Path of the "Main.rs" file where Agent's code will be saved.
const API_SCHEMA_PATH: &str = r"D:\Rust\rusty_gemini\schemas\api_schema.json"; // Path of the ".json" file where API Schema will be stored.

// Extend ai function to encourage specific output
// Forces the model to prouduce ouput similar to function
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Text {
    let ai_function_str: &str = ai_func(func_input);

    let msg: String = format!(
        "Function: {}
         INSTRUCTION: You are a function printer. You ONLY print the results of functions.
         Nothing else. No commentary. Here is the input to the function: {}.
         Print out what the function will return.",
        ai_function_str, func_input
    );

    Text { text: msg }
}

// Perform a call to LLM
pub async fn ai_task_request(
    msg_content: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extended AI Function
    let extended_msg: Text = extend_ai_function(function_pass, &msg_content);

    // Print Current Status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM Response
    let llm_response: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Return Success or try again
    match llm_response {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

// Perform a call to LLM - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_content: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_content, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to Decode ai response from serde_json");
    return decoded_response;
}

// Check whether request url is vaild
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let res: reqwest::Response = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

// Get Code Template
pub fn read_code_templates_content() -> String {
    let path: String = CODE_TEMPLATE_PATH.to_string();
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exe Main Code
pub fn read_exe_main_content() -> String {
    let path: String = EXEC_MAIN_PATH.to_string();
    fs::read_to_string(path).expect("Failed to read main.rs file")
}

// Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = EXEC_MAIN_PATH.to_string();
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = API_SCHEMA_PATH.to_string();
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints file");
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ai_functions::aifunc_architect::print_project_scope;

    #[test]
    fn tests_extend_ai_function() {
        let extended_msg: Text = extend_ai_function(
            print_project_scope,
            "build a website that lets users translate input into any language of their choosing",
        );
        dbg!(&extended_msg);
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String = "Build me a webserver for making todo api".to_string();

        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            print_project_scope,
        )
        .await;

        dbg!(&res);

        assert!(res.len() > 10);
    }
}
