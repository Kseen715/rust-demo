#include <stdio.h>
#include <time.h>

#include "lib.h"

double pow_c(double x, int y) {
    double result = 1;
    for (int i = 0; i < y; i++) {
        result *= x;
    }
    return result;
}

int main() {
    printf("Hello from C!\n");
    print_rs();
    print_str("Hello from C using Rust's DLL string pass!");

    // Start measuring time
    int n = 64000;
    double base = 1.0000000001;
    clock_t start, end;
    double cpu_time_used;
    double sum = 0;
    start = clock();

    for (int i = 0; i < n; i++) {
        sum += pow_c(base, i);
    }

    // End measuring time
    end = clock();

    printf("[C] The sum of the first %d powers of %lf is %lf\n", n, base, sum);

    cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;

    printf("[C] Time taken to calculate: %f seconds\n", cpu_time_used);
    sum = 0;

    // Rust Fibonacci
    start = clock();
    for (int i = 0; i < n; i++) {
        sum += pow_rs(base, i);
    }
    end = clock();

    printf("[Rust] The sum of the first %d powers of %lf is %lf\n", n, base, sum);

    cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;

    printf("[Rust] Time taken to calculate: %f seconds\n", cpu_time_used);

    return 0;
}