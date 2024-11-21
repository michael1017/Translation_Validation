
struct _142818701327899198193217796487002500807 {
  int * data;
  unsigned long int len;
};

int f_gold_c  ( int a [ ], int n, int k );
int _RNvCskLTludxeex8_40ARRAY_ELEMENT_MOVED_K_USING_SINGLE_MOVES6f_gold  (struct _142818701327899198193217796487002500807 e, unsigned long int n, unsigned long int k);

int comp_main() {

    int a;
    __CPROVER_assume(a > 0);
    int b;
    __CPROVER_assume(b >= 0);
    int c;
    int d[c];

    struct _142818701327899198193217796487002500807 e = { .data = d, .len = c };

    int result_c = f_gold_c(d, a, b);
    int result_rust = _RNvCskLTludxeex8_40ARRAY_ELEMENT_MOVED_K_USING_SINGLE_MOVES6f_gold(e, a, b);

    __CPROVER_assert(result_c == result_rust, "The results are not the same");

}