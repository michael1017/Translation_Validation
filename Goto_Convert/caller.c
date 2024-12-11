int sum_c(int a, int b);
int sum_rs(int a, int b);

int main(void) {
    int a, b;
    int res1 = sum_c(a, b);
    int res2 = sum_rs(a, b);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}
