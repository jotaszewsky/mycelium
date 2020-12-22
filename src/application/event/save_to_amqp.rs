use plugins::amqp::publish;
use application::Observer;
use application::Value;

pub struct SaveToAmqp {
    url: String,
    queue: String
}

impl SaveToAmqp {
    pub fn new(url: String, queue: String) -> SaveToAmqp {
        SaveToAmqp { url: url, queue: queue }
    }
}

impl Observer for SaveToAmqp {
    fn on_notify(&mut self, value: &Value) -> () {
        publish(&self.url, &self.queue, &value.data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;

    #[test]
    fn constructor_forward_parameter() {
        assert_eq!(
            SaveToAmqp::new(String::from("localhost"), String::from("queue")).url,
            String::from("localhost")
        );
        assert_eq!(
            SaveToAmqp::new(String::from("localhost"), String::from("queue")).queue,
            String::from("queue")
        );
    }

    // how mock ?
    #[test]
    fn notify_save() {
        assert!(true);
    }
}