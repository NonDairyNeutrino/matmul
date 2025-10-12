#include <stdio.h>

// loop through elements of a vector and print each of them
void print_vector(int dim, int vec[dim]) {
  for (int i = 0; i < dim; i++) {
    printf("%d ", vec[i]);
  }
  printf("\n");
}

// print the elements of a integer matrix
void print_matrix(int dim, int mat[dim][dim]) {
  // loop through rows of matrix, printing each of them
  for (int i = 0; i < dim; i++) {
    // treat each row of the matrix as a vector
    int* vec = mat[i];
    print_vector(dim, vec);
  }
  printf("\n");
}

