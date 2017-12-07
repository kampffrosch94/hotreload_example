extern crate base;
use base::NumberProvider;
struct MyProvider;
const NR: u32 = 140;
impl NumberProvider for MyProvider {
    fn get(&mut self) -> u32 {
        NR
    }
}
#[no_mangle]
pub fn get_message() -> Box<NumberProvider> {
    Box::new(MyProvider{})
}
