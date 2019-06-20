use rdkafka::consumer::{ConsumerContext, StreamConsumer, MessageStream, DefaultConsumerContext};
use futures::{Stream, Async, Poll};
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::error::{KafkaError, RDKafkaError};
use std::time::Duration;
use serde::de::DeserializeOwned;
use rdkafka::ClientConfig;
use rdkafka::config::{FromClientConfigAndContext, RDKafkaLogLevel};

pub struct KafkaConfig (ClientConfig);

impl KafkaConfig {
    pub fn new() -> KafkaConfig {
        KafkaConfig(ClientConfig::new())
    }

    pub fn set<'a>(&'a mut self, key: &str, value: &str) -> &'a mut KafkaConfig {
        let KafkaConfig(client_config) = self;
        client_config.set(key, value);
        self
    }

    pub fn set_log_level(&mut self, log_level: RDKafkaLogLevel) -> &mut KafkaConfig {
        let KafkaConfig(client_config) = self;
        client_config.set_log_level(log_level);
        self
    }

}

pub struct KafkaStream<'a, C>
    where C: ConsumerContext + 'static,
{
    message_stream: MessageStream<'a, C>,
}

impl<C> KafkaStream<'static, C>
    where C: ConsumerContext + 'static
{
    pub fn messages<'a, 'de, V>(&'a mut self) -> impl Stream +'a
        where V: DeserializeOwned,
    {
        self.map(move |m|
            m.payload().map(|b| serde_json::from_slice::<V>(b)))
    }
}

impl<'a, C> Stream for KafkaStream<'a, C>
    where C: ConsumerContext + 'static,
{
    type Item = BorrowedMessage<'a>;
    type Error = KafkaError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, KafkaError> {
        match self.message_stream.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(Some(Ok(v)))) => Ok(Async::Ready(Some(v))),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::Ready(Some(Err(KafkaError::NoMessageReceived)))) => Err(KafkaError::NoMessageReceived),
            Ok(Async::Ready(Some(Err(e)))) => Err(e),
            Err(_) => Err(KafkaError::MessageConsumption(RDKafkaError::Fail)),
        }
    }
}

#[must_use = "Consumer polling thread will stop immediately if unused"]
pub struct KafkaConsumer<C: ConsumerContext + 'static = DefaultConsumerContext> {
    stream_consumer: StreamConsumer<C>,
}

impl<C> KafkaConsumer<C>
    where C: ConsumerContext + 'static,
{
    pub fn start(&self) -> KafkaStream<C> {
        KafkaStream {
            message_stream: self.stream_consumer.start(),
        }
    }

    pub fn start_with(&self, poll_interval: Duration, no_message_error: bool) -> KafkaStream<C> {
        KafkaStream {
            message_stream: self.stream_consumer.start_with(poll_interval, no_message_error)
        }
    }

    pub fn stop(&self) {
        self.stream_consumer.stop()
    }
}

pub trait DefaultKafkaClient
{
    fn create_consumer(&self) -> Result<KafkaConsumer<DefaultConsumerContext>, KafkaError>;
}

pub trait KafkaClient<C>
    where C: ConsumerContext + 'static
{
    fn create_consumer_with_context(&self, context: C) -> Result<KafkaConsumer<C>, KafkaError>;
}

impl DefaultKafkaClient for KafkaConfig
{
    fn create_consumer(&self) -> Result<KafkaConsumer, KafkaError> {
        self.create_consumer_with_context(DefaultConsumerContext)
    }
}

impl<'a, C> KafkaClient<C> for KafkaConfig
    where C: ConsumerContext + 'static
{
    fn create_consumer_with_context(&self, context: C) -> Result<KafkaConsumer<C>, KafkaError> {
        let KafkaConfig(client_config) = self;
        StreamConsumer::from_config_and_context(client_config, context)
            .map(|c| KafkaConsumer{ stream_consumer: c})
    }
}
