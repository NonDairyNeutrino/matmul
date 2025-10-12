typedef int (*func_t) (int);

void update_matrix(func_t foo, int dim, int mat[dim][dim]);
void map_matrix(func_t foo, int dim, int inmat[dim][dim], int outmat[dim][dim]);
void zero_matrix(int dim, int mat[dim][dim]);
void rand_matrix(int dim, int mat[dim][dim]);
void fill_matrix(int dim, int mat[dim][dim], int fill);
void matmul(int dimA, int matA[dimA][dimA], int dimB, int matB[dimB][dimB], int matC[dimA][dimB]);
void sqrmatmul(int dim, int matA[dim][dim], int matB[dim][dim], int matC[dim][dim]);
