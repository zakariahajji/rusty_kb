mod retriever;
use retriever::{load_knowledge_base, retrieve};

fn main() {
    let query = "mucus";

    match load_knowledge_base("./kb/medical.csv") {
        Ok(knowledge_base) => {
            let responses = retrieve(&knowledge_base, query);
            for response in responses {
                println!("Title: {}", response.title);
                println!("Text: {}\n", response.text);
            }
        }
        Err(e) => {
            eprintln!("Error loading knowledge base: {}", e);
        }
    }
}
