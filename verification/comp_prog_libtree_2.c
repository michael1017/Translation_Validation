
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

struct _181176718737685558184493180065612954960 {
    size_t len;
    size_t cap;
    char *ptr;
};

struct _193321403262341067951085538045071135451 {
  struct _181176718737685558184493180065612954960 vec;
};


void utoa_c(char *str, size_t v);
struct _193321403262341067951085538045071135451 utoa_rust(unsigned long int v);

int comp_main() {

    size_t v;
    __CPROVER_assume(v < 100);
    char str_c[20];

    utoa_c(str_c, v);
    struct _193321403262341067951085538045071135451 output = utoa_rust(v);

    __CPROVER_assert(strncmp(str_c, output.vec.ptr, 20) == 0, "Are the returned strings similar?");


}