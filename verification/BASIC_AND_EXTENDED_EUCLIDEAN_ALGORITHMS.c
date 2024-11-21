// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


int f_gold_c ( int a, int b ) {
  if ( a == 0 ) return b;
  return f_gold_c ( b % a, a );
}
