int f_gold_c ( double h, double m );
int f_gold ( double h, double m );

int main() {
    double h, m;

    __CPROVER_assume( !(h < 0 || m < 0 || h > 12 || m > 60) );
    int res1 = f_gold_c ( h, m );
    int res2 = f_gold ( h, m );
    __CPROVER_assert(res1 == res2, "Programs are equal");
}