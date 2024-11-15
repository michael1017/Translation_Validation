int sum1(int a, int b);
int _RNvCsaks48gR9Qj8_3sum4sum2(int a, int b);

int main(void) {
    int a, b;
    int res1 = sum1(a, b);
    int res2 = _RNvCsaks48gR9Qj8_3sum4sum2(a, b);
    __CPROVER_assert(res1 == res2, "sum1(a, b) == sum2(a, b)");
}
