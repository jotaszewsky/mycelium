use plugins::amqp::Amqp;
use application::{Observer, Value};

pub struct SaveToAmqp {
    amqp: Amqp,
    exchange: String,
    routing_key: String
}

impl SaveToAmqp {
    pub fn new(url: String, exchange: String, routing_key: String) -> SaveToAmqp {
        SaveToAmqp { amqp: Amqp::new(&url), exchange, routing_key }
    }
}

impl Observer for SaveToAmqp {
    fn on_notify(&mut self, value: &Value) -> () {
        self.amqp.publish(&self.exchange, &self.routing_key, &value.data, &value.header).unwrap();
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;

    #[test]
    #[should_panic]
    fn connect_error() {
        SaveToAmqp::new(String::from("amqp://localhost"), String::from("exchange"), String::from("routing_key"));
    }

    // how mock ?
    #[test]
    fn notify_save() {
        assert!(true);
    }
}