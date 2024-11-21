#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define FNV_OFFSET 14695981039346656037UL
#define FNV_PRIME 1099511628211UL

uint64_t hash_key_c(const char* key) {
    uint64_t hash = FNV_OFFSET;
    for (const char* p = key; *p; p++) {
        uint64_t val = (uint64_t)(unsigned char)(*p);
        hash ^= val;
        hash *= FNV_PRIME;
    }
    return hash;
}