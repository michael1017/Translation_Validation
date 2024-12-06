#include <stdint.h>
float f_gold ( float x );
float _RNvCsg4TKAiUvDp6_17AREA_OF_A_HEXAGON6f_gold( float x );

int compare_double_bits(float x, float y) {
    ////////// Quick Bit compare //////////
    // For 64 bit double to 64 bit int
    // Then we can treat it as normal int and compare

    uint32_t ix, iy;
    memcpy(&ix, &x, sizeof(float));
    memcpy(&iy, &y, sizeof(float));
    return ix == iy;
}

int main(void) {
    float a;
    float res1 = f_gold(a);
    float res2 = _RNvCsg4TKAiUvDp6_17AREA_OF_A_HEXAGON6f_gold(a);
    __CPROVER_assert(compare_double_bits(res1, res2), "Programs are equal");
}