use std::sync::{Arc, Mutex};
use application::Observer;
use application::Value;

pub struct EventSource {
    observers: Vec<Arc<Mutex<dyn Observer>>>,
}

impl EventSource {
    pub fn new() -> EventSource {
        EventSource {
            observers: vec![],
        }
    }

    pub fn notify(&self, value: Value) -> () {
        for observer in self.observers.clone() {
            let mut observer = observer.lock().unwrap();
            observer.on_notify(&value);
        }
    }

    pub fn register_observer(&mut self, observer: Arc<Mutex<dyn Observer>>) -> () {
        self.observers.push(observer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    * How create test doubles in rust?
    * solution ?
    * Test double, i think
    */
    pub struct SaveToAssertMock {
        pub assert: String
    }

    impl Observer for SaveToAssertMock {
        fn on_notify(&mut self, value: &Value) -> () {
            assert!(true);
            assert_eq!(value.data, self.assert);
        }
    }
    /*
    * end of test double
    */

    #[test]
    fn constructor_observers_vector_empty() {
        assert_eq!(EventSource::new().observers.len(), 0 );
    }

    #[test]
    fn register_one_observer() {
        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        assert_eq!(event_source.observers.len(), 1 );
    }

    #[test]
    fn register_multiple_observers() {
        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        assert_eq!(event_source.observers.len(), 2 );
    }

    #[test]
    fn notify_no_observers() {
        let event_source: EventSource = EventSource::new();
        event_source.notify(Value { data: String::from("test") });
        assert!(true);
    }

    #[test]
    fn notify_one_observer() {
        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.notify(Value { data: String::from("test") });
    }

    #[test]
    fn notify_multiple_observers() {
        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.notify(Value { data: String::from("test") });
    }
}