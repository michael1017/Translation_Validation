
float f_gold_c ( int a );
float f_gold_r (int a );

int comp_main() {

    int a;
    float result_c = f_gold_c(a);
    float result_rust = f_gold_r(a);
    __CPROVER_assert(result_c == result_rust, 
            "Are the results the same?");

}