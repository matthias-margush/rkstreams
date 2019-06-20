use rkstreams::topic::Topic;
use rkstreams::topic;
use rdkafka::config::RDKafkaLogLevel;
use rkstreams::client::consumer::DefaultKafkaClient;
use rkstreams::client::consumer::KafkaConfig;

fn main() {
    let customer_topic: Topic<&serde_bytes::Bytes, &serde_bytes::Bytes> = topic::topic("customer");

    let consumer = KafkaConfig::new()
        .set("group.id", "groupid")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_consumer()
        .expect("Consumer creation failed")
        .start();


    println!("Customer topic: {:?}", customer_topic);
}
