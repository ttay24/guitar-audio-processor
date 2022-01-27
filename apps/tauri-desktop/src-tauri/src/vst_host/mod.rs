extern crate vst;

pub mod test;

use std::sync::{Arc, Mutex};
use std::path::Path;

use vst::host::{Host, PluginLoader};
//use vst::plugin::Plugin;

struct TestHost;

impl Host for TestHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

pub fn test_vst() {
    println!("running test vst");

    // let host = Arc::new(Mutex::new(TestHost));
    let path = Path::new("K:/My Files/Music/VSTPlugins 64 bit");

    let vst_dir_exists: bool = path.is_dir();

    if vst_dir_exists {
        println!("found path \"{}\"", path.display());

        // let mut loader = PluginLoader::load(path, host.clone()).unwrap();
        // let mut instance = loader.instance().unwrap();

        // println!("Loaded {}", instance.get_info().name);
    }
    else {
        println!("path \"{}\" doesn't exist", path.display());
    }
}