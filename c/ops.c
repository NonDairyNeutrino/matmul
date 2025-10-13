// type for functions taking and returning single integers
#define DEBUG 0
#if DEBUG
  #include <stdio.h>
#endif
#include <threads.h>
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

// take the dot product of two vectors
int dot(int dim, int vecA[dim], int vecB[dim]) {
  int dp = 0;
  for (int i = 0; i < dim; i++) {
    dp += vecA[i] * vecB[i];
  }
  return dp;
}

// multiply two matrices and store the result in a given matrix
void matmul(int dimA, int matA[dimA][dimA], int dimB, int matB[dimB][dimB], int matC[dimA][dimB]) {
  for (int i = 0; i < dimA; i++) {
    for (int j = 0; j < dimB; j++) {
      matC[i][j] = 0; // make sure to zero out the result matrix
      int* row = matA[i];
      int* col = matB[][j] // TODO: add matrix transpose
    }
  }
}

// multiply two square matrices and store the result in a given matrix
void sqrmatmul(int dim, int matA[dim][dim], int matB[dim][dim], int matC[dim][dim]) {
  matmul(dim, matA, dim, matB, matC);
}


