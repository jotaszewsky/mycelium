pub trait Temporary {
    fn store(&mut self, key: String, data: Vec<u8>) -> Result<(), ()>;
    fn read(&mut self, key: String) -> Result<Vec<u8>, std::io::Error>;
    fn clear(&mut self, key: String) -> ();
}
