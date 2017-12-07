pub trait NumberProvider {
    fn get(&mut self) -> u32;
}
