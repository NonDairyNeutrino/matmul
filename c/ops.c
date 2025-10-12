// type for functions taking and returning single integers
#include <stdlib.h>
#include "ops.h"

// apply a function to each element of a matrix, modifying it in-place
void update_matrix(func_t foo, int dim, int mat[dim][dim]) {
  for (int i = 0; i < dim; i++) {
    for (int j = 0; j < dim; j++) {
      mat[i][j] = foo(mat[i][j]);
    }
  }
}

// apply a function to each element of a matrix and store the result in given matrix
void map_matrix(func_t foo, int dim, int inmat[dim][dim], int outmat[dim][dim]) {
  for (int i = 0; i < dim; i++) {
    for (int j = 0; j < dim; j++) {
      outmat[i][j] = foo(inmat[i][j]);
    }
  }
}

// set each element of a matrix to a given integer
void fill_matrix(int dim, int mat[dim][dim], int fill) {
  for (int i = 0; i < dim; i++) {
    for (int j = 0; j < dim; j++) {
      mat[i][j] = fill;
    }
  }
}

// set each element of a matrix to zero
void zero_matrix(int dim, int mat[dim][dim]) {
  fill_matrix(dim, mat, 0);
}

// set each element of a matrix to a random integer
void rand_matrix(int dim, int mat[dim][dim]) {
  for (int i = 0; i < dim; i++) {
    for (int j = 0; j < dim; j++) {
      mat[i][j] = random();
    }
  }
}

// multiply two matrices and store the result in a given matrix
void matmul(int dimA, int matA[dimA][dimA], int dimB, int matB[dimB][dimB], int matC[dimA][dimB]) {
  for (int i = 0; i < dimA; i++) {
    for (int j = 0; j < dimB; j++) {
      for (int k = 0; k < dimB; k++) {
        matC[i][j] += matA[i][k] * matB[k][j];
      }
    }
  }
}

// multiply two square matrices and store the result in a given matrix
void sqrmatmul(int dim, int matA[dim][dim], int matB[dim][dim], int matC[dim][dim]) {
  for (int i = 0; i < dim; i++) {
    for (int j = 0; j < dim; j++) {
      for (int k = 0; k < dim; k++) {
        matC[i][j] += matA[i][k] * matB[k][j];
      }
    }
  }
}
