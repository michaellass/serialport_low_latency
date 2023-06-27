use bindgen::{Builder, CargoCallbacks};
use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = Builder::default()
        .header("bindings.h")
        .allowlist_type("serial_struct")
        .allowlist_var("TIOCGSERIAL")
        .allowlist_var("TIOCSSERIAL")
        .allowlist_var("ASYNC_LOW_LATENCY")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
