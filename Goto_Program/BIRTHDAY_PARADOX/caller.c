typedef struct _142818701327899198193217796487002500807 {
  int * data;
  unsigned long int len;
} rsIntArr;


int f_gold_c ( double p );
int f_gold ( double p );

int main() {
    double p;


    int res1 = f_gold_c ( p );
    int res2 = f_gold ( p );
    __CPROVER_assert(res1 == res2, "Programs are equal");
}