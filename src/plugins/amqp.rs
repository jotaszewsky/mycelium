extern crate amiquip;
use self::amiquip::{AmqpValue, Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result, FieldTable, Exchange, Publish, AmqpProperties};

extern crate serde;
extern crate serde_json;

use std::collections::{HashMap, BTreeMap};
use application::event_source::EventSource;
use application::Value;

pub fn consume(
    url: &str,
    queue_name: &str,
    queue_arguments: Option<String>,
    acknowledgement: Option<Acknowledgements>,
    count: usize,
    prefetch_count: u16,
    event_source: EventSource
) -> Result<()> {
    let mut connection = Connection::insecure_open(url)?;
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(
        queue_name,
        QueueDeclareOptions {
            durable: true,
            arguments: build_field_table(queue_arguments),
            ..QueueDeclareOptions::default()
        }
    )?;

    channel.qos(0, prefetch_count, false)?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let headers: String = serde_json::to_string(&delivery.properties.headers()).unwrap();
                event_source.notify(Value {
                    data: body.to_string(),
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
    connection.close()
}

pub fn publish(url: &str, queue_name: &str, message: &str, header: &Option<String>) -> Result<()> {
    // change to not open connection every time
    let mut connection = Connection::insecure_open(url)?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    match header {
        Some(header) => exchange.publish(Publish::with_properties(message.as_bytes(), queue_name, build_properties(header)))?,
        None => exchange.publish(Publish::new(message.as_bytes(), queue_name))?
    }
    connection.close()
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Acknowledgements {
    ack,
    nack,
    reject,
    nack_requeue
}

fn build_field_table(queue_arguments: Option<String>) -> FieldTable {
    match queue_arguments {
        Some(json) => build_arguments(&json),
        None => FieldTable::new()
    }
}

fn build_properties(header: &String) -> AmqpProperties {
    let headers: BTreeMap<String, AmqpValue> = serde_json::from_str(header).unwrap();
    AmqpProperties::default().with_headers(headers)
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
struct Field {
    pub value: String,
    pub typedef: FieldTypes
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
enum FieldTypes {
    string,
    int
}

fn build_arguments(queue_arguments: &str) -> FieldTable {
    let mut args = FieldTable::new();
    let key_values: HashMap<String, Field> = serde_json::from_str(queue_arguments).unwrap();
    for (key, value) in key_values {
        match value.typedef {
            FieldTypes::string => args.insert(key, AmqpValue::LongString(String::from(value.value))),
            FieldTypes::int => args.insert(key, AmqpValue::ShortInt(value.value.parse::<i16>().unwrap()))
        };
    }
    return args;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn build_arguments_not_json_error() {
        build_field_table(Some(String::from("not_json?")));
    }

    #[test]
    fn build_arguments_string() {
        let fields: FieldTable = build_field_table(
            Some(
                String::from("{\"test1\": {\"value\": \"test value 1\", \"typedef\": \"string\"}, \"test2\": {\"value\": \"test value 2\", \"typedef\": \"string\"}}")
            )
        );
        let (first_key, first_value) = fields.iter().next().unwrap();
        assert_eq!((first_key, first_value), (&String::from("test1"), &AmqpValue::LongString(String::from("test value 1"))));
        let (second_key, second_value) = fields.iter().last().unwrap();
        assert_eq!((second_key, second_value), (&String::from("test2"), &AmqpValue::LongString(String::from("test value 2"))));
    }

    #[test]
    fn build_arguments_int() {
        let fields: FieldTable = build_field_table(
            Some(
                String::from("{\"test1\": {\"value\": \"1\", \"typedef\": \"int\"}, \"test2\": {\"value\": \"23423\", \"typedef\": \"int\"}}")
            )
        );
        let (first_key, first_value) = fields.iter().next().unwrap();
        assert_eq!((first_key, first_value), (&String::from("test1"), &AmqpValue::ShortInt(1)));
        let (second_key, second_value) = fields.iter().last().unwrap();
        assert_eq!((second_key, second_value), (&String::from("test2"), &AmqpValue::ShortInt(23423)));
    }

    #[test]
    fn build_arguments_mixed_int_string() {
        let fields: FieldTable = build_field_table(
            Some(
                String::from("{\"test1\": {\"value\": \"test value 1\", \"typedef\": \"string\"}, \"test2\": {\"value\": \"23423\", \"typedef\": \"int\"}}")
            )
        );
        let (first_key, first_value) = fields.iter().next().unwrap();
        assert_eq!((first_key, first_value), (&String::from("test1"), &AmqpValue::LongString(String::from("test value 1"))));
        let (second_key, second_value) = fields.iter().last().unwrap();
        assert_eq!((second_key, second_value), (&String::from("test2"), &AmqpValue::ShortInt(23423)));
    }

    #[test]
    #[should_panic]
    fn build_arguments_unknow_type_error() {
        build_field_table(Some(String::from("{\"test1\": {\"value\": \"0.2\", \"typedef\": \"float\"}}")));
    }

    #[test]
    fn build_empty_field_table_if_no_json() {
        let fields: FieldTable = build_field_table(None);
        assert_eq!(fields.len(), 0);
    }

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