#include <stdint.h>

typedef struct _142818701327899198193217796487002500807 {
  int * data;
  unsigned long int len;
} rsIntArr;


int compare_double_bits(double x, double y) {
    uint64_t ix, iy;
    memcpy(&ix, &x, sizeof(double));
    memcpy(&iy, &y, sizeof(double));
    return ix == iy;
}

double f_gold_c ( int side );
double f_gold ( int side );

int main() {
    int side;
    __CPROVER_assume(side == 789527);
    double res1 = f_gold_c(side);
    double res2 = f_gold(side);

    int res = compare_double_bits(res1, res2);

    __CPROVER_assert(res, "Programs are equal");
}

// side=789527 (00000000 00001100 00001100 00010111)
// res1=5.288087e+16 (01000011 01100111 01111011 11011011 11010010 10010111 10000001 10111111)
// res2=5.587290e+16 (01000011 01101000 11010000 00000011 10001101 00100011 01111111 10011001)
// res=0 (00000000 00000000 00000000 00000000)