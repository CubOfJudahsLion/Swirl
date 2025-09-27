#include <stdint.h>

int32_t add_c(int32_t a, int32_t b);

typedef struct {
    int32_t first, second;
} TwoInt32s;

TwoInt32s get_num_and_next();
