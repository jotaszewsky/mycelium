use std::sync::{Arc, Mutex};
use application::{Observer, Value, Pipe};

pub struct EventSource {
    observers: Vec<Arc<Mutex<dyn Observer>>>,
    pipe: Pipe
}

impl EventSource {
    pub fn new(pipe: Pipe) -> EventSource {
        EventSource {
            observers: vec![],
            pipe
        }
    }

    pub fn notify(&self, mut value: Value) -> () {
        for observer in self.observers.clone() {
            let mut observer = observer.lock().unwrap();
            if observer.allows_middleware() {
                value = self.pipe.dispatch(value);
            }
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
            assert_eq!(value.data, self.assert.clone().into_bytes().to_vec());
        }

        fn allows_middleware(&mut self) -> bool {
            true
        }
    }
    /*
    * end of test double
    */

    #[test]
    fn constructor_observers_vector_empty() {
        assert_eq!(
            EventSource::new(
                Pipe::new(
                    Vec::new()
                )
            ).observers.len(),
            0
        );
    }

    #[test]
    fn register_one_observer() {
        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        assert_eq!(event_source.observers.len(), 1 );
    }

    #[test]
    fn register_multiple_observers() {
        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        assert_eq!(event_source.observers.len(), 2 );
    }

    #[test]
    fn notify_no_observers() {
        let event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.notify(Value { data: String::from("test").into(), header: None });
        assert!(true);
    }

    #[test]
    fn notify_one_observer() {
        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.notify(Value { data: String::from("test").into(), header: None });
    }

    #[test]
    fn notify_multiple_observers() {
        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        event_source.notify(Value { data: String::from("test").into(), header: None });
    }
}