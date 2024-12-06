struct refstr {
  unsigned char * data;
  unsigned long int len;
};

typedef signed long int __int64_t;
typedef __int64_t int64_t;

int64_t FileSize(const char* path);
int64_t file_size(struct refstr path);

int main(void) {
    unsigned long int c = 1;
    unsigned char d[1];

    struct refstr e = { .data = d, .len = c };

    int64_t res1 = FileSize(d);
    int64_t res2 =  file_size(e);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}