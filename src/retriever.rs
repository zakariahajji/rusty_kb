use serde_json::{Value, from_str};
use std::fs::File;
use std::io::{self, Read};

pub struct Retriever {
    knowledge_base: Value,
}

impl Retriever {
    pub async fn new(kb_path: &str) -> io::Result<Self> {
        let mut file = File::open(kb_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let knowledge_base: Value = from_str(&contents).expect("JSON was not well-formatted");
        
        Ok(Self { knowledge_base })
    }

    pub async fn retrieve(&self, query: &str) -> Vec<Value> {

        vec![self.knowledge_base.clone()]
    }
}
