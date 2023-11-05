

pub struct Generator;

impl Generator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate(&self, inputs: &[serde_json::Value]) -> String {

        inputs
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
