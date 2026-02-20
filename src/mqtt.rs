use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS, Transport};
use rustls::ClientConfig;
use std::time::Duration;

use crate::messages::handle;

pub fn client_init() -> (AsyncClient, rumqttc::EventLoop) {
    let roots = rustls_native_certs::load_native_certs().expect("could not load native certs");

    let mut root_store = rustls::RootCertStore::empty();
    for cert in roots {
        root_store.add(cert).unwrap();
    }

    let tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let mut opts = MqttOptions::new("my-client", "mqtt.dkmondal.in", 8883);
    opts.set_credentials("dezire", "test1234");
    opts.set_keep_alive(Duration::from_secs(5));
    opts.set_transport(Transport::tls_with_config(tls_config.into()));

    AsyncClient::new(opts, 10)
}

pub async fn run(client: AsyncClient) {
    client
        .subscribe("khamoshchat/test/#", QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .publish("khamoshchat/test", QoS::AtLeastOnce, false, "hello!")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(5)).await;
}

pub async fn start_eventloop(mut eventloop: EventLoop, client: AsyncClient) {
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("Connected to MQTT broker");
                client
                    .subscribe("khamoshchat/test/#", QoS::AtLeastOnce)
                    .await
                    .unwrap();
                println!("Subscribed to all topics (#)");
            }
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let client = client.clone();
                tokio::spawn(async move {
                    handle(client, p).await;
                });
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Connection error: {}", e);
                eprintln!("Connection error (debug): {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
