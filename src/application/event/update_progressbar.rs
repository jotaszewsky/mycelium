use plugins::progressbar::Progressbar;
use application::{Observer, Value};

pub struct UpdateProgressBar {
    progressbar: Progressbar
}

impl UpdateProgressBar {
    pub fn new(count: usize) -> UpdateProgressBar {
        UpdateProgressBar { progressbar: Progressbar::new(count) }
    }
}

impl Observer for UpdateProgressBar {
    fn on_notify(&mut self, _value: &Value) -> () {
        self.progressbar.update().unwrap();
    }

    fn allows_middleware(&mut self) -> bool {
        false
    }
}
