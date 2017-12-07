extern crate libloading;
extern crate base;
use base::NumberProvider;
use std::mem::ManuallyDrop;

use libloading::Library;

const LIB_PATH: &'static str = "../app/target/debug/libapp.so";

struct Application {
    lib: ManuallyDrop<Library>,
    trait_object: ManuallyDrop<Box<NumberProvider>>
}

impl NumberProvider for Application{
    fn get(&self) -> u32 {
        self.trait_object.get()
    }
}

impl Application {
    fn new(lib: Library) -> Self {
        let trait_object = unsafe {
            let f = lib.get::<fn() -> Box<NumberProvider> >(
                b"get_message\0"
            ).unwrap();
            f()
        };
        Self {
            lib: ManuallyDrop::new(lib),
            trait_object: ManuallyDrop::new(trait_object),
        }
    }
}

impl Drop for Application{
    //noinspection RsDropRef
    fn drop(&mut self) {
        println!("Dropping the Library");
        unsafe {
            ManuallyDrop::drop(&mut self.trait_object);
            ManuallyDrop::drop(&mut self.lib);
        }
    }
}

fn main() {

    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap()
        .modified().unwrap();

    let mut app = Application::new(Library::new(LIB_PATH)
        .unwrap_or_else(|error| panic!("{}", error)));
    println!("message: {}", app.get());

    let dur = std::time::Duration::from_secs(1);
    loop {
        std::thread::sleep(dur);
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH)
                                  .map(|m| m.modified())
        {
            if modified > last_modified {
                drop(app);
                app = Application::new(Library::new(LIB_PATH).unwrap());
                last_modified = modified;
            }

        }
        println!("message: {}", app.get());
    }
}
