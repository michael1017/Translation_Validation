
float f_gold_c ( int a );
float _RNvCsfOtbjDXjUki_65AREA_OF_THE_CIRCLE_THAT_HAS_A_SQUARE_AND_A_CIRCLE_INSCRIBED_IN_IT6f_gold (int a );

int comp_main() {

    int a;

    __CPROVER_assume(a >= 0);

    float result_c = f_gold_c(a);
    float result_rust = _RNvCsfOtbjDXjUki_65AREA_OF_THE_CIRCLE_THAT_HAS_A_SQUARE_AND_A_CIRCLE_INSCRIBED_IN_IT6f_gold(a);

    __CPROVER_assert(result_c == result_rust, "Are the results the same?");

}