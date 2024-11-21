
int f_gold_c ( int a );
int _RNvCslWLVncWmzZz_46BELL_NUMBERS_NUMBER_OF_WAYS_TO_PARTITION_A_SET6f_gold (unsigned long int a );

int comp_main() {

    int a;
    __CPROVER_assume(a >= 0 && a < 2);
    unsigned long int c = a;

    int result_c = f_gold_c(a);
    int result_rust = _RNvCslWLVncWmzZz_46BELL_NUMBERS_NUMBER_OF_WAYS_TO_PARTITION_A_SET6f_gold(c);

    __CPROVER_assert(result_c == 1000, "Are the results the same?");

}