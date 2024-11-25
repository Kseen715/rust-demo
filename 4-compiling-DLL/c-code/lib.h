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