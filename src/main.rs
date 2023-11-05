mod retriever;
mod generator;
mod chatbot;

#[tokio::main]
async fn main() {
    let query = "What is Rust programming?"; // Example query
    let chatbot = chatbot::ChatBot::new("./kb/medical.csv").await;

    match chatbot.answer_question(query).await {
        Ok(response) => println!("Chatbot: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
