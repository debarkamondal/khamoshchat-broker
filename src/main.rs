mod mqtt;
mod messages;

#[tokio::main]
async fn main() {

    //connect and initiate mqtt event loop
    let (client, eventloop) = mqtt::client_init();
    mqtt::start_eventloop(eventloop, client).await;
}
