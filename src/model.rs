use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Subsystem {
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
    id: Option<String>,

    context: String,
    source: String,
    quote: String,
    description: String,
    
    subsystems: Vec<Subsystem>,
    concepts: Vec<Concept>,
}
