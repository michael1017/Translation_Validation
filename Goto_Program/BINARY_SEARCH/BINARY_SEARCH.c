// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int min(int x, int y) { return (x < y)? x: y; }
int max(int x, int y) { return (x > y)? x: y; }
int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

int f_gold_c ( int arr [ ], int l, int r, int x ) {
  if ( r >= l ) {
    int mid = l + ( r - l ) / 2;
    if ( arr [ mid ] == x ) return mid;
    if ( arr [ mid ] > x ) return f_gold_c ( arr, l, mid - 1, x );
    return f_gold_c ( arr, mid + 1, r, x );
  }
  return - 1;
}


int f_filled ( int arr [ ], int l, int r, int x ) {}
