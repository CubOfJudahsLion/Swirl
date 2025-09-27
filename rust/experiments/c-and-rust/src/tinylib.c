// A very tiny C library for testing.
#include "tinylib.h"

//  Rust function. Just produces a random number.
extern int32_t gimme_a_number();

//  Simplest test ever.
int32_t add_c(int32_t a, int32_t b)
{
    return a + b;
}

//  Just gets a number from Rust and returns it along with its successor.
TwoInt32s get_num_and_next()
{
    int32_t const first = gimme_a_number();
    return (TwoInt32s){.first = first, .second = first + 1};
}
