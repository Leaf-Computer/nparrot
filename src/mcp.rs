use crate::utils::wait_for_message;
use nostr_sdk::prelude::*;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ErrorData, ServerHandler,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendMessageRequest {
    #[schemars(description = "The message to send to the user")]
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Chat {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
    client: Client,
    our_pubkey: PublicKey,
    target_pubkey: PublicKey,
}

#[tool_router]
impl Chat {
    pub fn new(client: Client, our_pubkey: PublicKey, target_pubkey: PublicKey) -> Self {
        Self {
            tool_router: Self::tool_router(),
            client,
            our_pubkey,
            target_pubkey,
        }
    }

    #[tool(description = "Send a message to the user")]
    async fn send(
        &self,
        Parameters(SendMessageRequest { message }): Parameters<SendMessageRequest>,
    ) -> Result<String, ErrorData> {
        self.client
            .send_private_msg(self.target_pubkey, message, [])
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        Ok("Sent message".to_string())
    }

    #[tool(description = "Listen and wait for the user's next message")]
    async fn wait(&self) -> Result<String, ErrorData> {
        wait_for_message(&self.client, &self.our_pubkey, &self.target_pubkey)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }
}

#[tool_handler]
impl ServerHandler for Chat {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("This server provides tools for talking to a specific user over the Nostr protocol via encrypted DMs.")
    }
}
