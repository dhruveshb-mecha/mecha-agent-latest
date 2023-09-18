use anyhow::{bail, Result};
use nats_client::NatsClient;
use serde::{Serialize, Deserialize};
use settings::AgentSettings;
use crypto::x509::sign_with_private_key;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;
use crypto::base64::b64_encode;

#[derive(Debug)]
pub struct Messaging {
    scope: MessagingScope,
    settings: AgentSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingAuthTokenRequest {
    id: String,
    #[serde(rename = "type")]
    _type: MessagingAuthTokenType,
    scope: MessagingScope,
    nonce: String,
    signed_noce: String,
    public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessagingScope {
    #[serde(rename = "sys")]
    System,
    #[serde(rename = "app")]
    App,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessagingAuthTokenType {
    #[serde(rename = "device")]
    Device,
}


impl Messaging {
    pub fn new(scope: MessagingScope) -> Self {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => AgentSettings::default(),
        };
        Self { scope, settings }
    }

    /**
     * The Messaging Service connection will perform the following
     * 1. Authenticate
     * 2. Create NATs Client
     * 3. Connect NATs client with token
     * 4. Check for connection event, re-connect if necessary
     */
    pub async fn connect(&self) -> Result<bool> {
        let trace_id = find_current_trace_id();
        tracing::trace!(trace_id, task = "connect", "init");

        let nats_url = "nats://localhost:4222";

        let mut nats_client = NatsClient::new(nats_url);
        let nats_user_public_key = nats_client.user_public_key.clone();
        let auth_token = match self.authenticate(&nats_user_public_key) {
            Ok(t) => t,
            Err(e) => bail!(e),
        };

        nats_client.connect(&auth_token).await?;

        Ok(true)
    }

    /**
     * Performs messaging authentication and returns the JWT. Auth procedure is
     * 1. Requests nonce from the server
     * 2. Signs the nonce using the Device Key
     * 3. Requests the token from the server
     */
    fn authenticate(&self, user_public_key: &str) -> Result<String> {
        // Step 1: Get Device ID
        let device_id = String::from("deviceId");

        // Step 2: Get Nonce from Server
        let nonce = match self.get_auth_nonce() {
            Ok(n) => n,
            Err(e) => bail!(e),
        };

        // Step 3: Sign the nonce
        let nonce_sign = match self.sign_nonce(&nonce) {
            Ok(n) => n,
            Err(e) => bail!(e),
        };

        let token = match self.get_auth_token(
            self.scope.clone(),
            &device_id,
            &nonce,
            &nonce_sign,
        ) {
            Ok(t) => t,
            Err(e) => bail!(e),
        };

        // Ok(token)
        Ok(String::from(String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJzdWIiOiJVQUc0NldLWlNNUDY2VjRHSkhFMk1GT0ZQSlRNM1hURzVVR082TFdEVEo0SDJRWURNT1RSWkROTCIsIm5hbWUiOiJ1c2VyXzIiLCJpYXQiOjE2OTQ4MDY0NTYsImlzcyI6IkFDM0FCWVRZRVZTMjM2MlhFNjVVNFZPREZESTQ1V0tHT0tPNUY3VjdaV0JMUkwyWEZOQUxUTzZOIiwiZXhwIjoxMDAwMDAwMDAwMCwibmF0cyI6eyJwdWIiOnsiYWxsb3ciOlsiZm9vIl0sImRlbnkiOltdfSwic3ViIjp7ImFsbG93IjpbImZvbyJdLCJkZW55IjpbXX0sInN1YnMiOi0xLCJkYXRhIjotMSwicGF5bG9hZCI6LTEsImlzc3Vlcl9hY2NvdW50IjoiQUFNNTRIVzRKTElWVTJPU0hNMzRVT1RRVEtENjVMNTIyVEFSMzZITkNQUzdBS1NHVDJFQ0Q0QVIiLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.74rE4mtIsXGV19Di0eCrKM-MMpVnPs8EZFCkIPUImOqQpZ0QZU4ox-Em3NFUSYLzNRpL46GqAKrPyoE29xb6Dg")))
    }

    fn sign_nonce(&self, nonce: &str) -> Result<String> {
        let private_key_path = &self.settings.provisioning.paths.device.private_key;
        let signed_nonce = match sign_with_private_key(&private_key_path, nonce.as_bytes()) {
            Ok(s) => s,
            Err(e) => bail!(e),
        };

        let encoded_signed_nonce = b64_encode(signed_nonce);
        Ok(encoded_signed_nonce)
    }

    fn get_auth_nonce(&self) -> Result<String> {
        let trace_id = find_current_trace_id();
        tracing::trace!(trace_id, task = "request_nonce", "init");

        // TODO
        // Send API call to /messaging/auth/nonce
        // The nonce api will generate an encrypted nonce with AES
        
        // Handle the errors from API and also internal errors

        Ok(String::from("Z3Tt80mW1hz67TVDeIUG1odkE41Oq8yU"))
    }

    fn get_auth_token(&self, scope: MessagingScope, id: &str, nonce: &str, signed_nonce: &str) -> Result<String> {
        // TODO
        // Send id, nonce and signed_nonce to the server
        // Get the token back
        Ok(String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJzdWIiOiJVQU5NWVRHUFFNRFFSWDZHWUxNUUdUSlNCSk9ESE9WWkw0Q1pDS1RRWDRITTI3WElBM1dORzRKUCIsIm5hbWUiOiJ1c2VyXzIiLCJpYXQiOjE2OTQ4MDY0NTYsImlzcyI6IkFDM0FCWVRZRVZTMjM2MlhFNjVVNFZPREZESTQ1V0tHT0tPNUY3VjdaV0JMUkwyWEZOQUxUTzZOIiwiZXhwIjoxMDAwMDAwMDAwMCwibmF0cyI6eyJwdWIiOnsiYWxsb3ciOlsiZm9vIl0sImRlbnkiOltdfSwic3ViIjp7ImFsbG93IjpbImZvbyJdLCJkZW55IjpbXX0sInN1YnMiOi0xLCJkYXRhIjotMSwicGF5bG9hZCI6LTEsImlzc3Vlcl9hY2NvdW50IjoiQUFNNTRIVzRKTElWVTJPU0hNMzRVT1RRVEtENjVMNTIyVEFSMzZITkNQUzdBS1NHVDJFQ0Q0QVIiLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.YkY1fvK-F5Ku-QFLs2Jl0MYpBLp5D00zGzswOHmF4AYVcH3tQ9SR4kr-QxxUIxRfLiGlxbnijYbe7ljeSzjBCg"))
    } 
}
