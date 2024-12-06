#include <sys/types.h>
struct refstr {
  unsigned char * data;
  unsigned long int len;
};

struct csv_parser {
  int pstate;         /* Parser state */
  int quoted;         /* Is the current field a quoted field? */
  size_t spaces;      /* Number of continious spaces after quote or in a non-quoted field */
  unsigned char * entry_buf;   /* Entry buffer */
  size_t entry_pos;   /* Current position in entry_buf (and current size of entry) */
  size_t entry_size;  /* Size of entry buffer */
  int status;         /* Operation status */
  unsigned char options;
  unsigned char quote_char;
  unsigned char delim_char;
  int (*is_space)(unsigned char);
  int (*is_term)(unsigned char);
  size_t blk_size;
  void *(*malloc_func)(size_t);
  void *(*realloc_func)(void *, size_t);
  void (*free_func)(void *);
};

typedef signed long int __int64_t;
typedef __int64_t int64_t;

int c_csv_get_opts(const struct csv_parser *p);
int csv_get_opts(const struct csv_parser *p);

int main(void) {
    const struct csv_parser p;

    int res1 = c_csv_get_opts(&p);
    int res2 = csv_get_opts(&p);

    __CPROVER_assert(res1 == res2, "Programs are equal");
}