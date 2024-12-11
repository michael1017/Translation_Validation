typedef struct _142818701327899198193217796487002500807 {
  int * data;
  unsigned long int len;
} rsIntArr;


int f_gold_c ( int arr [ ], int l, int r, int x );
int f_gold (rsIntArr arr, unsigned long int l, unsigned long int r, int x );

int main() {
    int arrLen;
    int arr[arrLen];
    rsIntArr rsArr = { .data = arr, .len = arrLen };
    int l, r, x;

    __CPROVER_assume(l >= 0 && r >= 0 && arrLen >= 0);

    int res1 = f_gold_c(arr, l, r, x);
    int res2 = f_gold(rsArr, l, r, x);
    __CPROVER_assert(res1 == res2, "Programs are equal");
}