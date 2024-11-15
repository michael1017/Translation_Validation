int f_gold (int x);
int _RNvCs1TZo2aAjjjQ_25ADD_1_TO_A_GIVEN_NUMBER_16f_gold(int x);

int main(void) {
    int a;
    int res1 = f_gold(a);
    int res2 = _RNvCs1TZo2aAjjjQ_25ADD_1_TO_A_GIVEN_NUMBER_16f_gold(a);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}