use plugins::mongodb::MongoDB;
use application::{Observer, Value};


pub struct SaveToMongoDB {
    mongodb: MongoDB
}

impl SaveToMongoDB {
    pub fn new(dsn: String, database: String, collection: String) -> SaveToMongoDB {
        SaveToMongoDB { mongodb: MongoDB::new(dsn, database, collection) }
    }
}

impl Observer for SaveToMongoDB {
    fn on_notify(&mut self, value: &Value) -> () {
        self.mongodb.publish(value).unwrap();
    }

    fn allows_middleware(&mut self) -> bool {
        true
    }
}
