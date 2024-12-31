use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=./src");
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    println!("cargo:include=/usr/local/include");

    cc::Build
        ::new()
        .warnings(false)
        .extra_warnings(false)
        .define("__GPIO_H_IMPLEMENTATION__", Some("1"))
        .file("c/gpio.c")
        .compile("gpio"); // outputs `.a`

    println!("cargo:rustc-link-lib=wiringPi");
    println!("cargo:rustc-link-lib=gpio");

    // Run bindgen to generate bindings for wiringPi
    let bindings = bindgen::Builder
        ::default()
        .header("c/gpio.c")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I./src")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to output file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("wiringPi.rs")).expect("Failed to write bindings");

    // Run bindgen to generate bindings for gpio
    let bindings = bindgen::Builder
        ::default()
        .header("c/gpio.c")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I./src")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to output file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("gpio.rs")).expect("Failed to write bindings");

    // Add required attributes to suppress warnings
    println!("cargo:warning=Adding allow attributes to generated bindings");
    println!("cargo:rustc-attr=allow(non_upper_case_globals)");
    println!("cargo:rustc-attr=allow(non_camel_case_types)");
    println!("cargo:rustc-attr=allow(non_snake_case)");
    println!("cargo:rustc-attr=allow(improper_ctypes)");
    println!("cargo:rustc-attr=allow(dead_code)");
}
