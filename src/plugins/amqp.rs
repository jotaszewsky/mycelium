extern crate amiquip;
use self::amiquip::{AmqpValue, Connection, Channel, ConsumerMessage, ConsumerOptions, Result, Publish, AmqpProperties};

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use application::event_source::EventSource;
use application::Value;

pub struct Amqp {
    connection: Connection,
    channel: Channel
}

impl Amqp {

    pub fn new(url: &String) -> Amqp {
        let mut connection = Connection::insecure_open(url).unwrap();
        let channel = connection.open_channel(None).unwrap();
        Amqp { connection, channel }
    }

    pub fn publish(&mut self, exchange: &String, routing_key: &String, message: &[u8], header: &Option<String>) -> Result<()> {
        match header {
            Some(header) => self.channel.basic_publish(exchange, Publish::with_properties(message, routing_key, build_properties(header)))?,
            None => self.channel.basic_publish(exchange, Publish::new(message, routing_key))?
        }
        Ok(())
    }

    pub fn consume(
        &mut self,
        queue: String,
        acknowledgement: Option<Acknowledgements>,
        count: usize,
        prefetch_count: u16,
        event_source: EventSource
    ) -> Result<()> {
        self.channel.qos(0, prefetch_count, false)?;
        let consumer = self.channel.basic_consume(
            queue,
            ConsumerOptions::default()
        )?;
        println!("Waiting for messages. Press Ctrl-C to exit.");

        for (i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = &delivery.body;
                    let headers: String = serde_json::to_string(&delivery.properties.headers()).unwrap();
                    event_source.notify(Value {
                        data: body.to_vec(),
                        header: Some(headers)
                    });
                    match acknowledgement {
                        Some(Acknowledgements::ack) => consumer.ack(delivery)?,
                        Some(Acknowledgements::nack) => consumer.nack(delivery, false)?,
                        Some(Acknowledgements::reject) => consumer.reject(delivery, false)?,
                        Some(Acknowledgements::nack_requeue) => consumer.nack(delivery, true)?,
                        None => consumer.ack(delivery)?
                    }
                    if count != 0 && (i+1) >= count {
                        println!("Consumer ended after {:?} listings!", count);
                        break;
                    }
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        self.connection.close()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Acknowledgements {
    ack,
    nack,
    reject,
    nack_requeue
}

fn build_properties(header: &String) -> AmqpProperties {
    let headers: BTreeMap<String, AmqpValue> = serde_json::from_str(header).unwrap();
    AmqpProperties::default().with_headers(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_build_properties_no_json() {
        let properties: AmqpProperties = build_properties(&String::from("sf"));
        match properties.headers() {
            Some(headers) => {
                for (_key, _value) in headers {
                    assert!(false);
                }
            },
            None => panic!()
        }
    }

    #[test]
    fn test_build_properties() {
        let properties: AmqpProperties = build_properties(&String::from("{\"test\": {\"LongString\": \"test_value_1\"}}"));
        match properties.headers() {
            Some(headers) => {
                for (key, value) in headers {
                    assert_eq!(key, &String::from("test"));
                    assert_eq!(value, &AmqpValue::LongString(String::from("test_value_1")));
                }
            },
            None => panic!()
        }
    }
}