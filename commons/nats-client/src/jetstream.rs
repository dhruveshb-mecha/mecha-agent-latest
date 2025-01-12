use anyhow::{bail, Result};
use async_nats::jetstream::{
    consumer::{pull::Config, Consumer},
    stream::Stream,
};

use crate::errors::{NatsClientError, NatsClientErrorCodes};

#[derive(Clone, Debug)]
pub struct JetStreamClient {
    pub client: async_nats::Client,
}

impl JetStreamClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { client }
    }

    pub async fn get_stream(&self, stream_name: String) -> Result<Stream> {
        let jetstream = async_nats::jetstream::new(self.client.clone());
        let stream = match jetstream.get_stream(stream_name).await {
            Ok(s) => s,
            Err(e) => {
                bail!(NatsClientError::new(
                    NatsClientErrorCodes::GetStreamError,
                    format!("get stream error {:?}", e),
                ))
            }
        };
        Ok(stream)
    }
    pub async fn create_consumer(
        &self,
        stream: Stream,
        filter_subject: String,
        consumer_name: String,
    ) -> Result<Consumer<Config>> {
        let consumer = match stream
            .create_consumer(async_nats::jetstream::consumer::pull::Config {
                name: Some(consumer_name),
                filter_subject: filter_subject,
                ..Default::default()
            })
            .await
        {
            Ok(s) => s,
            Err(e) => bail!(NatsClientError::new(
                NatsClientErrorCodes::CreateConsumerError,
                format!("error creating consumer - {}", e),
            )),
        };
        Ok(consumer)
    }
}
