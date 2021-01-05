use plugins::amqp::Amqp;
use application::{Observer, Value};

pub struct SaveToAmqp {
    amqp: Amqp
}

impl SaveToAmqp {
    pub fn new(url: String, queue: String) -> SaveToAmqp {
        SaveToAmqp { amqp: Amqp::new(&url, queue, None) }
    }
}

impl Observer for SaveToAmqp {
    fn on_notify(&mut self, value: &Value) -> () {
        self.amqp.publish(&value.data, &value.header).unwrap();
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;

    #[test]
    #[should_panic]
    fn connect_error() {
        SaveToAmqp::new(String::from("amqp://localhost"), String::from("queue"));
    }

    // how mock ?
    #[test]
    fn notify_save() {
        assert!(true);
    }
}