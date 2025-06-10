use plugins::amqp::Amqp;
use application::{Observer, Value};

pub struct SaveToAmqp {
    amqp: Amqp
}

impl SaveToAmqp {
    pub fn new(url: String, exchange: String, routing_key: String) -> SaveToAmqp {
        SaveToAmqp { amqp: Amqp::new_write(&url, exchange, routing_key) }
    }
}

impl Observer for SaveToAmqp {
    fn on_notify(&mut self, value: &Value) -> () {
        self.amqp.publish(value).unwrap();
    }

    fn allows_middleware(&mut self) -> bool {
        true
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