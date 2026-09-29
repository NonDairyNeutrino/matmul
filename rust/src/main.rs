// use std::ops::{AddAssign, Mul};

mod linalg {
    use std::thread;

    // Allocates a vector of zeros except for a 1 in a single index
    pub fn sel_vec(dim: usize, p: usize) -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(dim);
        for _ in 0..v.capacity() {
            v.push(0.0);
        }
        v[p] = 1.0;
        return v;
    }

    // Allocates a vector of zeros except ones on the diagonal
    pub fn id_mat(dim: usize) -> Vec<Vec<f64>> {
        let mut mat: Vec<Vec<f64>> = Vec::with_capacity(dim);
        for i in 0..mat.capacity() {
            let row: Vec<f64> = sel_vec(dim, i);
            mat.push(row);
        }
        return mat;
    }

    pub fn zero_mat(dim: usize) -> Vec<Vec<f64>> {
        let mut mat: Vec<Vec<f64>> = Vec::with_capacity(dim);
        for _ in 0..mat.capacity() {
            let mut row: Vec<f64> = Vec::with_capacity(dim);
            for _ in 0..row.capacity() {
                row.push(0.0);
            }
            mat.push(row);
        }
        return mat;
    }

    // fn dot<T: Mul + AddAssign>(x: Vec<T>, y: Vec<T>) {
    pub fn dot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        // assert_eq!(x.len(), y.len())
        let n: usize = x.len();
        let mut acc: f64 = 0.0;
        for i in 0..n {
            acc += x[i] * y[i];
        }
        return acc;
    }

    pub fn matvecmul(mat: &Vec<Vec<f64>>, x: &Vec<f64>, y: &mut Vec<f64>) {
        let nrows: usize = mat.len();

        let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(nrows);
        let num_threads: usize = thread::available_parallelism().unwrap().into();
        let row_slice_length = nrows / num_threads;

        // partition matrix into collections of slices of the original
        let mut row_slices = Vec::new();
        let mut y_slices = Vec::new();

        for i in 0..num_threads {
            let start: usize = i * row_slice_length;
            let end: usize = (i + 1) * row_slice_length;

            let row_slice: &[Vec<f64>] = &mat[start..end];
            row_slices.push(row_slice);

            let y_slice: &[f64] = &y[start..end];
            y_slices.push(y_slice);
        }

        // thread over rows slices
        for t in 0..num_threads {
            let handle = thread::spawn(|| todo!());

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>) {
        let nrows: usize = a.len();
        let ncols: usize = b[0].len();

        // allocate and initialize input and output column vectors
        let mut bcol: Vec<f64> = Vec::with_capacity(nrows);
        let mut ccol: Vec<f64> = Vec::with_capacity(nrows);
        for _ in 0..nrows {
            bcol.push(0.0);
            ccol.push(0.0);
        }

        // to work with rows instead of columns we can allocate and reuse only a single vector
        for j in 0..ncols {
            // set values for the column
            for i in 0..nrows {
                bcol[i] = b[i][j];
            }
            // use the column in a matrix-vector multiplication
            matvecmul(a, &bcol, &mut ccol);
            // set the values of the ouput matrix for the column
            for i in 0..nrows {
                c[i][j] = ccol[i];
            }
        }
    }
}

fn main() {
    use linalg::*;
    const DIM: usize = 3;
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let mut y: Vec<f64> = vec![0.0, 0.0, 0.0];
    println!("{x:?}.{y:?} = {}", dot(&x, &y));

    let a: Vec<Vec<f64>> = id_mat(DIM);
    matvecmul(&a, &x, &mut y);
    println!("{a:?}.{x:?} = {y:?}");

    let b: Vec<Vec<f64>> = id_mat(DIM);
    let mut c: Vec<Vec<f64>> = zero_mat(DIM);
    matmul(&a, &b, &mut c);

    println!("{a:?}.{b:?} = {c:?}");
}
