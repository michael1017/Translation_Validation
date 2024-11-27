struct refstr {
  unsigned char * data;
  unsigned long int len;
};


static int ParseInt(const char* s, int low, int high, int* result);
int parse_int(struct refstr s,  int low, int high);

int main(void) {
    int low, high;
    unsigned long int s_len;
    unsigned char s[s_len];

    struct refstr rust_str = { .data = s, .len = s_len };

    int res1;
    ParseInt(s, low, high, &res1);
    int res2 =  parse_int(rust_str, low, high);

    __CPROVER_assert(res1 == res2, "Programs are equal");
}