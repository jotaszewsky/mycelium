extern crate amiquip;
use self::amiquip::{AmqpValue, Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result, FieldTable, Exchange, Publish};

extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use application::event_source::EventSource;
use application::Value;

pub fn consume(
    url: &str,
    queue_name: &str,
    queue_arguments: Option<String>,
    from_acknowledgements: Option<Acknowledgements>,
    count: usize,
    prefetch_count: u16,
    event_source: EventSource
) -> Result<()> {
    let mut connection = Connection::insecure_open(url)?;
    let channel = connection.open_channel(None)?;

    let args = build_field_table(queue_arguments);

    let queue = channel.queue_declare(
        queue_name,
        QueueDeclareOptions {
            durable: true,
            arguments: args,
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
                event_source.notify(Value {
                    data: body.to_string()
                });
                // do it better ?
                match from_acknowledgements {
                    Some(Acknowledgements::ack) => consumer.ack(delivery)?,
                    Some(Acknowledgements::nack) => consumer.nack(delivery, false)?,
                    Some(Acknowledgements::reject) => consumer.reject(delivery, false)?,
                    // Check how
                    //Some(Acknowledgements::recover) => consumer.recover(delivery)?,
                    //Some(Acknowledgements::recover_requeue) => consumer.recover(delivery, true)?,
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

// wyjac poza albo powielic, nie mozna korzystac tu i tam
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Acknowledgements {
    ack,
    nack,
    reject,
    // Check how ?
    // recover,
    // recover_requeue,
    nack_requeue
}

fn build_field_table(queue_arguments: Option<String>) -> FieldTable {
    match queue_arguments {
        Some(json) => build_arguments(&json),
        None => FieldTable::new()
    }
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

pub fn publish(url: &str, queue_name: &str, message: &str) -> Result<()> {
    let mut connection = Connection::insecure_open(url)?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::new(message.as_bytes(), queue_name))?;
    connection.close()
}