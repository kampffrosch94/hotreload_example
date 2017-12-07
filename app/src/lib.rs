extern crate base;
use base::NumberProvider;
struct MyProvider;
const nr: u32 = 105;
impl NumberProvider for MyProvider {
    fn get(&self) -> u32 {
        nr
    }
}
#[no_mangle]
pub fn get_message() -> Box<NumberProvider> {
    Box::new(MyProvider{})
}
