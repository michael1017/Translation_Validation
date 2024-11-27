#include <sys/types.h>
struct refstr {
  unsigned char * data;
  unsigned long int len;
};

typedef struct {
  size_t len;
  char *alloc;
  char *data;
} buffer_t;


buffer_t* buffer_new_with_size(size_t n);
buffer_t* new_with_size(size_t n);

int main(void) {
    size_t n;

    buffer_t* res1 = buffer_new_with_size(n);
    buffer_t* res2 = new_with_size(n);

    __CPROVER_assert(res1->len == res2->len, "Programs' len are equal");

    // for (int i=0; i<n; i++) {
    //   __CPROVER_assert(res1->alloc[i] == res2->alloc[i], "Programs' alloc are equal");
    //   __CPROVER_assert(res1->data[i] == res2->data[i], "Programs' data are equal");
    // }
}