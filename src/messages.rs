use rumqttc::AsyncClient;


pub async fn handle(_client: AsyncClient, msg: rumqttc::Publish) {
    let payload = std::str::from_utf8(&msg.payload).unwrap_or("invalid utf8");
    println!("Received on {}: {}", msg.topic, payload);

    // process and forward
    // client
    //     .publish("recipient/topic", QoS::AtLeastOnce, false, payload)
    //     .await
    //     .unwrap();
}
