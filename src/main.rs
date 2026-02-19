// mod errors;
mod mqtt;

#[tokio::main]
async fn main() {
    let (client, eventloop) = mqtt::client_init();
    mqtt::start_eventloop(eventloop, client).await;
}
