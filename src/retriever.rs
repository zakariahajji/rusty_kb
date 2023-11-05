use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub title: String,
    pub text: String,
}

pub fn load_knowledge_base(kb_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(kb_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut knowledge_base: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        knowledge_base.push(record);
    }

    Ok(knowledge_base)
}

pub fn retrieve<'a>(knowledge_base: &'a [Record], query: &str) -> Vec<&'a Record> {
    let mut matches: Vec<&'a Record> = Vec::new();

    for record in knowledge_base {
        if record.title.contains(query) || record.text.contains(query) {
            matches.push(record);
        }
    }

    matches
}
