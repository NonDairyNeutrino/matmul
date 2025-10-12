// matmul.c
// Multithreaded matrix multiplication using new C-standard threading

#include <stdio.h>
#include <stdlib.h>
#include "print.h"
#include "ops.h"

int square(int x) {
  return x*x;
}

int main(int argc, char* argv[]) {
  int dim = atoi(argv[1]);

  // initialize a matrix with garbage values
  int mat[dim][dim];
  printf("Initial matrix\n");
  print_matrix(dim, mat);

  // replace matrix values with a value
  fill_matrix(dim, mat, 2);
  printf("Filled matrix\n");
  print_matrix(dim, mat);

  // scan through a matrix and apply a function to each element
  int sqrmat[dim][dim];
  map_matrix(square, dim, mat, sqrmat);
  printf("Elemental squared matrix\n");
  print_matrix(dim, sqrmat);

  //
  int matsqrd[dim][dim];
  sqrmatmul(dim, mat, mat, matsqrd);
  printf("Matrix squared\n");
  print_matrix(dim, matsqrd);

  return 0;
}

