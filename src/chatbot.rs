use crate::retriever::Retriever;
use crate::generator::Generator;

pub struct ChatBot {
    retriever: Retriever,
    generator: Generator,
}

impl ChatBot {
    pub async fn new(kb_path: &str) -> Self {
        let retriever = Retriever::new(kb_path).await.expect("Failed to create retriever");
        let generator = Generator::new();
        
        Self { retriever, generator }
    }

    pub async fn answer_question(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let retrieved_docs = self.retriever.retrieve(query).await;
        let answer = self.generator.generate(&retrieved_docs).await;
        Ok(answer)
    }
}
