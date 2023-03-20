use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Subsystem {
    Phonetics,
    Phonology,
    Morphology,
    Syntax,
    Semantics,
    Pragmatics,
    Discourse,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Concept {
    pub concept: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    pub context: String,
    pub source: String,
    pub quote: String,
    pub description: String,
    
    pub subsystems: Vec<Subsystem>,
    pub concepts: Vec<Concept>,
}

#[derive(Serialize, Deserialize)]
pub struct QuoteResponse {
    pub id: String,
    pub quote: Quote,
}
