use rusoto_core::{Region};
use rusoto_ecs::{Ecs, EcsClient, ListTasksRequest};

#[tokio::main]
async fn main() {
    let client = EcsClient::new(Region::UsEast1);
    match client.list_tasks(ListTasksRequest {
        cluster: Some("production".to_string()),
        service_name: Some("magento-cron-prd".to_string()),
        ..Default::default()
    }).await.map(|resp| {
        return resp.task_arns
    }) {
        Ok(Some(resp)) => println!("Success: {:#?}", resp),
        Ok(None) => println!("No task arns"),
        Err(err) => println!("Error: {}", err)
    };
}
