
int f_gold_c ( int a, int b );
int _RNvCsf8CrQ4W8dBK_39BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS6f_gold ( int a, int b );

int comp_main() {

    int a;
    int b;

    int result_c = f_gold_c(a, b);
    int result_rust = _RNvCsf8CrQ4W8dBK_39BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS6f_gold(a, b);

    __CPROVER_assert(result_c == result_rust, "The results are not the same");

}