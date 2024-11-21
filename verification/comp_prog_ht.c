
#include <stdint.h>
#include <stdlib.h>

uint64_t hash_key_c(const char* key);
uint64_t hash_key_rust(const char* key);

int comp_main() {

    char key[6];
    for (int i = 0; i < 5; i++) {
        key[i] = key[i] >> 1;
    }
    // __CPROVER_assume(key[0] > 0);
    // __CPROVER_assume(key[1] > 0);
    // __CPROVER_assume(key[2] > 0);
    // __CPROVER_assume(key[3] > 0);
    // __CPROVER_assume(key[4] > 0);
    key[5] = '\0';

    uint64_t result_c = hash_key_c(key);
    uint64_t result_rust = hash_key_rust(key);

    __CPROVER_assert(result_c == result_rust, "Are the results the same?");

}