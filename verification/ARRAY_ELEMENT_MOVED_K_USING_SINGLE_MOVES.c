// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


int f_gold_c ( int a [ ], int n, int k ) {
  if ( k >= n - 1 ) return n;
  int best = 0, times = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( a [ i ] > best ) {
      best = a [ i ];
      if ( i ) times = 1;
    }
    else times += 1;
    if ( times >= k ) return best;
  }
  return best;
}
