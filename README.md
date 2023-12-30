# Rusty - Automate Your Code Generation using Gemini

Rusty is a Rust-based CLI application that generates, builds, and tests backend server code in Rust based on user prompts.

## Getting Started

### Create .env file

Within the created .env file, paste the following:

```plaintext
API_KEY = YOUR_GEMINI_API_KEY
```

### Update Path

Update constants in the src/helpers/general path.

### Build and Run Project

```shell
cargo build
cargo run
```

## Working

The project tries to mimic industrial code development with LLM agents.

### LLM Agents:

- Managing Agent
- Solution Architect Agent
- Backend Agent
- _Can be extended depending on use cases..._

### Managing Agent

- Converts user prompts into a more concise and technical version.
- Manages the other agents.

### Solution Architect Agent

- Defines the project scope.
- Fetches and tests external APIs for the backend agent.

### Backend Agent

- Builds initial Rust code using given prompts and code templates (Junior Developer).
- Improves the initial Rust code and verifies that it meets all user requirements (Senior Developer).
- Fixes broken or buggy code.
- Creates an API schema for frontend developers or end users.

## Prompt Format

For the actual prompts, check src/ai_functions and src/helpers/general. Here, I have discussed the general format of the actual prompt.

```
Function: fn function_name(input) { // Comments specifying what the code will output. }
Instruction: Given the input: [YOUR_INPUT], output what the above code outputs.
```

We are prompting the model to output the actual result of the function. However, the function contains only commands. <br>
Therefore, models with high reasoning are required for this project.

## Observations

For this project, I used both GPT-4 and Gemini Pro. Here are my observations so far; however, these might change in the future.

- In terms of quality, I felt GPT-4 had an upper hand over Gemini Pro. I couldn't find a perfect temperature for Gemini; with a higher temperature, there is much hallucination, and with a lower temperature, it strictly adheres to the template.
- In terms of speed, Gemini Pro is super fast.
- Both the models perform poorly in debugging and code fixing.
- If you are a person who loves to code along with Rusty, use Gemini.

## Future Improvements

- Add more Task-Specific LLM Agents.
- Introduce Chain of Thoughts prompting.
- Write tooling to test all methods; for now, the code only tests the GET Method.
- Add front-end LLM Agents.
- Introduce local open-source LLMs.

## Special Mention

Shaun McDonogh [YouTube](https://www.youtube.com/@coderaiders8722/videos) (Thank you for the amazing Rust Course 💖) <br>
Karun A [GitHub](https://github.com/Karun842002) (Thank you for the GPT-4 API key 💖)
