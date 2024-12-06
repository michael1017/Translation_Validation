
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

struct _313047857359679364757414693850068124965 {
  unsigned long int * data;
  unsigned long int len;
};

int is_ascending_order_c(uint64_t *v, size_t n);
_Bool is_ascending_order_rust(struct _313047857359679364757414693850068124965 arg);

int comp_main() {

    // Define input arguments and call the functions and compare their results
    unsigned int n;
    __CPROVER_assume(n < 5);
    uint64_t v[n];

    struct _313047857359679364757414693850068124965 arg = { .data = v, .len = n };
    
    int result_c = is_ascending_order_c(v, n);
    int result_rust = (int) is_ascending_order_rust(arg);

    __CPROVER_assert(result_c == result_rust, "Are the results the same?");


}