use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parts {
    pub parts: Vec<Text>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  GenerationConfig {
    pub temperature: f32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contents {
    pub contents: Vec<Parts>,
    pub generationConfig: GenerationConfig
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub content: Parts,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AIResponse {
    pub candidates: Vec<Content>,
}