//! Khamoshchat RMQTT Plugin
//!
//! A custom RMQTT broker plugin that intercepts and logs client lifecycle events:
//! - **ClientConnect**: logs connecting client IP addresses
//! - **ClientAuthenticate**: logs client ID and username during authentication
//! - **ClientSubscribeCheckAcl**: logs and allows subscription ACL checks
//! - **SessionSubscribed**: logs successful subscriptions
//! - **MessagePublish**: logs published message topics and payloads

use async_trait::async_trait;
use rmqtt::{
    Result,
    context::ServerContext,
    hook::{Handler, HookResult, Parameter, Register, ReturnType, Type},
    macros::Plugin,
    plugin::{PackageInfo, Plugin},
};

/// Log target for all plugin log messages, enabling filtered output via
/// `RUST_LOG=khamoshchat=info` or similar.
const LOG_TARGET: &str = "khamoshchat";

/// The core plugin struct registered with the RMQTT broker.
///
/// Hooks into client lifecycle events and logs them for observability.
#[derive(Plugin)]
pub struct KhamoshchatPlugin {
    scx: ServerContext,
    register: Box<dyn Register>,
}

impl KhamoshchatPlugin {
    /// Creates a new plugin instance, obtaining a hook register from the server context.
    #[inline]
    pub async fn new(scx: ServerContext, _name: &str) -> std::result::Result<Self, rmqtt::Error> {
        let register = scx.extends.hook_mgr().register();
        Ok(Self { scx, register })
    }
}

#[async_trait]
impl Plugin for KhamoshchatPlugin {
    #[inline]
    async fn init(&mut self) -> Result<()> {
        log::info!(target: LOG_TARGET, "{} init", self.name());
        let handler = Box::new(KhamoshchatHandler);

        self.register
            .add(Type::ClientAuthenticate, handler.clone())
            .await;
        self.register
            .add(Type::ClientConnect, handler.clone())
            .await;
        self.register
            .add(Type::ClientSubscribeCheckAcl, handler.clone())
            .await;
        self.register
            .add(Type::SessionSubscribed, handler.clone())
            .await;
        self.register.add(Type::MessagePublish, handler).await;

        Ok(())
    }

    #[inline]
    async fn start(&mut self) -> Result<()> {
        log::info!(target: LOG_TARGET, "{} start", self.name());
        self.register.start().await;
        Ok(())
    }

    #[inline]
    async fn stop(&mut self) -> Result<bool> {
        log::info!(target: LOG_TARGET, "{} stop", self.name());
        self.register.stop().await;
        Ok(true)
    }

    #[inline]
    async fn get_config(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({}))
    }
}

/// Event handler that logs MQTT client lifecycle events.
#[derive(Clone)]
struct KhamoshchatHandler;

#[async_trait]
impl Handler for KhamoshchatHandler {
    async fn hook(&self, param: &Parameter, acc: Option<HookResult>) -> ReturnType {
        match param {
            Parameter::ClientConnect(conn_info) => {
                log::info!(
                    target: LOG_TARGET,
                    "Client connecting from {:?}",
                    conn_info.ipaddress()
                );
            }
            Parameter::ClientAuthenticate(conn_info) => {
                let id = conn_info.id();
                let client_id = &id.client_id;
                let username = id.username_ref();
                log::info!(
                    target: LOG_TARGET,
                    "Client authenticate - client_id: {}, username: {}",
                    client_id,
                    username
                );
            }
            Parameter::ClientSubscribeCheckAcl(session, subscribe) => {
                let client_id = &session.id.client_id;
                log::info!(
                    target: LOG_TARGET,
                    "Client {} checking subscribe ACL for topic: {:?}",
                    client_id,
                    subscribe.topic_filter
                );

                // Allow all subscriptions by default
                let acl_result =
                    rmqtt::types::SubscribeAclResult::new_success(subscribe.opts.qos(), None);
                return (true, Some(HookResult::SubscribeAclResult(acl_result)));
            }
            Parameter::SessionSubscribed(session, subscribe) => {
                let client_id = &session.id.client_id;
                log::info!(
                    target: LOG_TARGET,
                    "Client {} successfully subscribed to: {:?}",
                    client_id,
                    subscribe.topic_filter
                );
            }
            Parameter::MessagePublish(_session, from, publish) => {
                let topic = &publish.topic;
                let payload = String::from_utf8_lossy(&publish.payload);
                log::info!(
                    target: LOG_TARGET,
                    "Message published on topic '{}' from '{:?}': {}",
                    topic,
                    from,
                    payload
                );
            }
            _ => {
                log::debug!(
                    target: LOG_TARGET,
                    "Unhandled parameter: {:?}",
                    param.get_type()
                );
            }
        }
        (true, acc)
    }
}

rmqtt::register!(KhamoshchatPlugin::new);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_is_clone() {
        let handler = KhamoshchatHandler;
        let _cloned = handler.clone();
    }

    #[test]
    fn log_target_is_set() {
        assert_eq!(LOG_TARGET, "khamoshchat");
    }
}
