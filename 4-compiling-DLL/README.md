# DLL Compilation in Rust and usage (in C/C++)

<p align="center">
  <img src="https://github.com/Kseen715/imgs/blob/main/sakura_kharune.png?raw=true" height="100"/>
</p>

## Running the code

1. Run `make -B run` in `c-code` directory to compile and run the C code. It will compile the Rust code and then compile the C code using the generated DLL.

The output should look like this (YMMV):

```JSON
"Windows_NT detected!"
cd ../ && (cargo build --release)
   Compiling compiling-dll v0.1.0 ({PATH_TO_PROJECT}\rust-demo\4-compiling-DLL)
    Finished `release` profile [optimized] target(s) in 0.65s
move ..\target\release\demodll.dll build\demodll.dll
Перемещено файлов:         1.
gcc -c -Wall -fPIC -std=c11 -O3 -I.  -D _WIN32 main.c  -o build/demodll.obj
gcc build/demodll.obj -o build/demodll.exe -ldemodll -L./build
cd build && demodll.exe && cd ../
Hello from C!
Hello from Rust's DLL!
Hello from C using Rust's DLL string pass!
[C] The sum of the first 64000 powers of 1.000000 is 64000.204797
[C] Time taken to calculate: 3.293000 seconds
[Rust] The sum of the first 64000 powers of 1.000000 is 64000.204797
[Rust] Time taken to calculate: 3.609000 seconds
```

## Explanation

This example demonstrates how to compile a Rust library as a DLL and use it in C code. The Rust code is compiled as a DLL and the C code is compiled using the generated DLL.

The Rust code is compiled as a DLL using the `dylib` crate type in the `Cargo.toml` file:

```TOML
[lib]
name = "demodll"
crate-type = ["dylib"]
```

The C code uses the Rust DLL by including the header file [`lib.h`](c-code/lib.h) and linking against the generated DLL:

```C
#ifndef RUST_FUNCTIONS_H
#define RUST_FUNCTIONS_H

#ifdef __cplusplus
extern "C" {
#endif

// Function to print a message from Rust
void print_rs(void);

// Function to print a string from Rust
void print_str(const char* s);

// Function to calculate the power of a number
double pow_rs(double b, int p);

#ifdef __cplusplus
}
#endif

#endif // RUST_FUNCTIONS_H
```

The C code uses the Rust functions by calling them directly. The Rust functions are declared in the header file `lib.h` and defined in the Rust code `lib.rs`.

The C code is compiled using the generated DLL by linking against it using the `-ldemodll` flag:

```bash
gcc build/demodll.obj -o build/demodll.exe -ldemodll -L./build
```

The C code is then able to call the Rust functions as if they were regular C functions.


