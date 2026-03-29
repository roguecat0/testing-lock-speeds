#include <stdio.h>
#include "math_ops.h"
#include "librust_ffi.h"

int main() {
  int a = rust_multiply(3,4);
  printf("hello world %d\n", a);
}
