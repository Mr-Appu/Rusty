use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent: &str, agent_statement: &str) {
        let mut stout: std::io::Stdout = stdout();

        // Decide the color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Green,
            Self::Issue => Color::Red,
        };

        // Print the Agent and Agent Statement
        stout.execute(SetForegroundColor(Color::DarkMagenta)).unwrap();
        print!("Agent: {}: ", agent);

        stout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset Color
        stout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in specific color
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read User Input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response !!!");

    user_response.trim().to_string()
}

// Get User Response to execute AI Gen Code
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();

    loop {
        // Print Question
        stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI. ");
        println!("Review your code and confirm you wish to continue.");
        stdout.execute(ResetColor).unwrap();

        // Options
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");
        stdout.execute(ResetColor).unwrap();

        // User Input
        let mut hum_res: String = String::new();
        stdin()
            .read_line(&mut hum_res)
            .expect("Failed to read response from User");

        let hum_res: String = hum_res.trim().to_lowercase();

        match hum_res.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'");
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_print_agent_message() {
        PrintCommand::AICall.print_agent_message("GEMINI", "Fetching Code !!!");
        PrintCommand::UnitTest.print_agent_message("GPT", "Testing !!!");
        PrintCommand::Issue.print_agent_message("ULTRON", "Error !!!");
        dbg!(confirm_safe_code());
    }
}
