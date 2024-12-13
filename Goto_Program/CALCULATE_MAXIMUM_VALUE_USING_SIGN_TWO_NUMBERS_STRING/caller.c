struct refstr {
  unsigned char * data;
  unsigned long int len;
};

int f_gold_c ( char str [] );
int f_gold ( struct refstr str );

int main() {
    int len;
    __CPROVER_assume( len >= 0 );

    char str[len];
    for (int i=0; i<len; i++) {
        __CPROVER_assume( str[i] >= 0 );
    }
    
    struct refstr data = {.data=str, .len=len};
    
    int res1 = f_gold_c ( str );
    int res2 = f_gold ( data );
    __CPROVER_assert(res1 == res2, "Programs are equal");
}