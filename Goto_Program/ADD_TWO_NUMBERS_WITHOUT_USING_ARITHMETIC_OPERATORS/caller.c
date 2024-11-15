int f_gold (int x, int y);
int _RNvCslop8WA0Gbbt_50ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS6f_gold(int x, int y);

int main(void) {
    int a, b;
    int res1 = f_gold(a, b);
    int res2 = _RNvCslop8WA0Gbbt_50ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS6f_gold(a, b);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}