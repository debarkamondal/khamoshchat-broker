mod plugin;

use plugin::KhamoshchatPlugin;
use rmqtt::{context::ServerContext, net::Builder, server::MqttServer, Result};

/// Default TCP listen port for the MQTT broker.
const DEFAULT_PORT: u16 = 1883;

/// Default bind address.
const DEFAULT_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    // Read optional host/port overrides from environment
    let host = std::env::var("BROKER_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let port: u16 = std::env::var("BROKER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    log::info!("Starting Khamoshchat RMQTT Broker...");

    // Create server context
    let scx = ServerContext::new().build().await;

    // Register the custom plugin
    let scx_clone = scx.clone();
    scx.plugins
        .register("khamoshchat-plugin", true, false, move || {
            let scx_inner = scx_clone.clone();
            Box::pin(async move {
                let plugin = KhamoshchatPlugin::new(scx_inner, "khamoshchat-plugin")
                    .await
                    .map_err(|e| rmqtt::Error::msg(format!("Plugin init failed: {}", e)))?;
                Ok(Box::new(plugin) as rmqtt::plugin::DynPlugin)
            })
        })
        .await
        .expect("Failed to register khamoshchat plugin");

    // Parse bind address
    let addr: std::net::IpAddr = host
        .parse()
        .unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));

    // Build and start MQTT server
    log::info!("Starting TCP listener on {}:{}", addr, port);
    let server = MqttServer::new(scx)
        .listener(
            Builder::new()
                .name("tcp")
                .laddr((addr, port).into())
                .bind()?
                .tcp()?,
        )
        .build();

    server.start();

    // Wait for shutdown signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    log::info!("Shutting down Khamoshchat RMQTT Broker...");

    Ok(())
}
