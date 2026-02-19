use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS, Transport};
use rustls::ClientConfig;
use std::time::Duration;

fn tls_config() -> ClientConfig {
    let roots = rustls_native_certs::load_native_certs().expect("could not load native certs");

    let mut root_store = rustls::RootCertStore::empty();
    for cert in roots {
        root_store.add(cert).unwrap();
    }

    ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

pub fn client_init() -> (AsyncClient, rumqttc::EventLoop) {
    let mut opts = MqttOptions::new("my-client", "mqtt.dkmondal.in", 8883);
    opts.set_credentials("dezire", "test1234");
    opts.set_keep_alive(Duration::from_secs(5));
    opts.set_transport(Transport::tls_with_config(tls_config().into()));

    // let tls_config = TlsConfiguration::default();
    // opts.set_transport(Transport::tls_with_config(tls_config));
    // let (client, mut eventloop) = AsyncClient::new(opts, 10);
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

async fn handle(_client: AsyncClient, msg: rumqttc::Publish) {
    let payload = std::str::from_utf8(&msg.payload).unwrap_or("invalid utf8");
    println!("Received on {}: {}", msg.topic, payload);

    // process and forward
    // client
    //     .publish("recipient/topic", QoS::AtLeastOnce, false, payload)
    //     .await
    //     .unwrap();
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
                handle(client.clone(), p).await;
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
// pub async fn start_eventloop(mut eventloop: rumqttc::EventLoop, client: AsyncClient) {
//     while let Ok(event) = eventloop.poll().await {
//         // if let Event::Incoming(Packet::Publish(p)) = event {
//         //     println!("Received: {:?}", p.payload);
//         // }
//         match event {
//             Event::Incoming(Packet::ConnAck(_)) => {
//                 println!("Connected to MQTT broker");
//             }
//             Event::Incoming(Packet::Publish(p)) => {
//                 handle(client.clone(), p).await;
//             }
//             _ => {}
//         }
//     }
// }
