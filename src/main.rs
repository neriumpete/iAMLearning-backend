use axum::{
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
struct Question {
    question: String,
    #[serde(rename = "choices")]
    options: Vec<String>,
    answer: String,
    explanation: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Topic {
    #[serde(default)]
    id: u32, // added field with default value if missing
    title: String,
    summary: String,
    questions: Vec<Question>,
}

#[derive(Deserialize, Serialize, Clone)]
struct StudyData {
    topics: Vec<Topic>,
}

// Handler to serve the study data as JSON
async fn get_topics() -> Json<StudyData> {
    let data = fs::read_to_string("acams_content.json")
        .expect("Unable to read acams_content.json");
    let mut study_data: StudyData = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    // Auto-increment IDs for topics that are missing (defaulted to 0)
    for (i, topic) in study_data.topics.iter_mut().enumerate() {
        if topic.id == 0 {
            topic.id = i as u32 + 1; // Assign id starting at 1
        }
    }

    Json(study_data)
}

#[tokio::main]
async fn main() {
    // Build the application with a route to /topics
    let app = Router::new().route("/topics", get(get_topics));
    println!("Server listening on http://localhost:3000");
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
