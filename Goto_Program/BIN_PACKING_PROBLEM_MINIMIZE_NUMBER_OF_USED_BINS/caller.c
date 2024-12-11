typedef struct _142818701327899198193217796487002500807 {
  int * data;
  unsigned long int len;
} rsIntArr;


int f_gold_c ( int weight [ ], int n, int c );
int f_gold (rsIntArr arr, unsigned long int n, int c);

int main() {
    int arrLen;
    int arr[arrLen];
    rsIntArr rsArr = { .data = arr, .len = arrLen };

    int n, c;

    // __CPROVER_assume(n >= 0 && arrLen >= 0);

    int res1 = f_gold_c(arr, n, c);
    int res2 = f_gold(rsArr, n, c);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}