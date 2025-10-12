// matmul.c
// Multithreaded matrix multiplication using new C-standard threading

#include <stdlib.h>
#include "print.h"
#include "ops.h"

int square(int x) {
  return x*x;
}

int main(int argc, char* argv[]) {
  int dim = atoi(argv[1]);
  int fill = atoi(argv[2]);
  int mat[dim][dim];
  int matsqrd[dim][dim];

  fill_matrix(dim, mat, fill);
  sqrmatmul(dim, mat, mat, matsqrd);
  print_matrix(dim, matsqrd);
  return 0;
}

