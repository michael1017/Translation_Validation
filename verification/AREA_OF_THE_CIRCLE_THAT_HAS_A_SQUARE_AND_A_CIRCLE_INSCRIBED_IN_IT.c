// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


#include <math.h>

float f_gold_c ( int a ) {
  float area = (M_PI * (float )a * (float)a )/4.0;
  return area;
}
