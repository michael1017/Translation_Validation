int f_gold ( int arr [ ], unsigned long int n, int x );
int _RNvCs1xAz186unTv_48ANALYSIS_OF_ALGORITHMS_SET_2_ASYMPTOTIC_ANALYSIS6f_gold( int arr [ ], unsigned long int n, int x );

int main(void) {
    unsigned long int a;
    int b;
    int arr[4];
    int res1 = f_gold(arr, a, b);
    int res2 = _RNvCs1xAz186unTv_48ANALYSIS_OF_ALGORITHMS_SET_2_ASYMPTOTIC_ANALYSIS6f_gold(arr, a, b);
    __CPROVER_assert(res1 == res2, "Programs are equal");

}