extern crate mongodb;
use self::mongodb::sync::{
    Client, Database, Collection
};
use self::mongodb::options::FindOptions;

extern crate serde;
extern crate serde_json;

extern crate bson;

use application::event_source::EventSource;
use application::Value;

pub struct MongoDB {
    collection: Collection
}

impl MongoDB {

    pub fn new(dsn: String, database_name: String, collection_name: String) -> MongoDB {
        let client: Client = Client::with_uri_str(&dsn).unwrap();
        let database: Database = client.database(&database_name);
        let collection: Collection = database.collection(&collection_name);
        MongoDB { collection }
    }

    pub fn publish(&mut self, value: &Value) -> Result<(), ()> {
        let _ = self.collection.insert_one(
            serde_json::from_slice(&value.data).unwrap(),
            None
        );
        Ok(())
    }

    pub fn consume(
        &mut self,
        count: usize,
        event_source: EventSource
    ) -> Result<(), ()> {
        let mut options: Option<FindOptions> = None;
        if count != 0 {
            options = Some(FindOptions::builder()
                .limit(count as i64)
                .build());
        }
        let cursor = self.collection.find(None, options).unwrap();
        for result in cursor {
            match result {
                Ok(document) => {
                    event_source.notify(Value {
                        data: serde_json::to_string(&document).unwrap().as_bytes().to_vec(),
                        header: None
                    });
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        if count != 0 {
            println!("Consumer ended after {:?} listings!", count);
        }
        Ok(())
    }
}
