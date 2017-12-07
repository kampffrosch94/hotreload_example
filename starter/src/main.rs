extern crate libloading;
extern crate base;

use base::NumberProvider;
use std::mem::ManuallyDrop;
use std::time::SystemTime;
use std::time::Instant;
use std::time::Duration;

use libloading::Library;

const LIB_PATH: &'static str = "../app/target/debug/libapp.so";

struct ApplicationWrapper {
    lib: ManuallyDrop<Library>,
    trait_object: ManuallyDrop<Box<NumberProvider>>,
    last_modified: SystemTime,
}

impl NumberProvider for ApplicationWrapper {
    fn get(&mut self) -> u32 {
        self.reload_if_necessary();
        self.trait_object.get()
    }
}

impl ApplicationWrapper {
    fn new() -> ApplicationWrapper {
        let last_modified: SystemTime = std::fs::metadata(LIB_PATH).unwrap()
            .modified().unwrap();
        let lib = Library::new(LIB_PATH)
            .unwrap();
        let trait_object = unsafe {
            let f = lib.get::<fn() -> Box<NumberProvider>>(
                b"get_message\0"
            ).unwrap();
            f()
        };
        ApplicationWrapper {
            lib: ManuallyDrop::new(lib),
            trait_object: ManuallyDrop::new(trait_object),
            last_modified,
        }
    }

    //noinspection RsDropRef
    fn reload_if_necessary(&mut self){
        let t = Instant::now();
        if let Ok(Ok(modified)) = std::fs::metadata(LIB_PATH)
            .map(|m| m.modified())
            {
                if modified > self.last_modified {
                    self.last_modified = modified;
                    //swap lib and trait_object
                    unsafe {
                        ManuallyDrop::drop(&mut self.trait_object);
                        ManuallyDrop::drop(&mut self.lib);
                        let mut lib = Library::new(LIB_PATH)
                            .unwrap();
                        let trait_object = {
                            let f = lib.get::<fn() -> Box<NumberProvider>>(
                                b"get_message\0"
                            ).unwrap();
                            f()
                        };
                        self.trait_object = ManuallyDrop::new(trait_object);
                        self.lib = ManuallyDrop::new(lib);
                    }
                }
            }
        let dur = t.elapsed();
        println!("Ms for check: {}", dur.subsec_nanos() as f64 / 1_000_000.0);
    }
}

impl Drop for ApplicationWrapper {
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
    let mut app = ApplicationWrapper::new();

    let dur = std::time::Duration::from_secs(1);
    loop {
        std::thread::sleep(dur);
        println!("message: {}", app.get());
    }
}
