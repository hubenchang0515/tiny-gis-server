pub trait Cache {
    fn has(&self, id: &str) -> bool;
    fn save(&mut self, id: &str, data: Vec<u8>);
    fn get(&self, id: &str) -> Option<&Vec<u8>>;
    fn delete(&mut self, id: &str);
}