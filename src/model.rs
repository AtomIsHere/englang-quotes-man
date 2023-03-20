use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Subsystem {
    Phonetics,
    Phonology,
    Morphology,
    Syntax,
    Semantics,
    Pragmatics,
    Discourse,
}

#[derive(Serialize, Deserialize)]
pub struct Concept {
    pub concept: String,
}

#[derive(Serialize, Deserialize)]
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
