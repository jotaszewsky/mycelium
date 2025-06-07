use application::Value;

pub trait Observer {
    fn on_notify(&mut self, value: &Value) -> ();

    fn allows_middleware(&mut self) -> bool;
}
