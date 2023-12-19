#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use crate::models::agent_manager::managing_agent::ManagingAgent;
use helpers::command_line::get_user_response;

#[tokio::main]
async fn main() {
    let user_req: String = get_user_response("What website are we developing today ?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(user_req)
        .await
        .expect("Project Manager Agent Failed !!!");

    manage_agent.execute_project().await;

    // dbg!(manage_agent);
}
