use std::println;
pub async fn process_task(task: &serde_json::Value) -> String {
    let payload = &task["payload"];
    println!("Processing task with payload: {:?}", payload);
    "completed".to_string()
}
