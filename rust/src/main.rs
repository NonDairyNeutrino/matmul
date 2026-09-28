// use std::ops::{AddAssign, Mul};

mod linalg {
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
        for row in 0..nrows {
            y[row] = dot(&mat[row], x);
        }
    }

    pub fn matmul(x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>, z: &mut Vec<Vec<f64>>) {
        let nrows: usize = x.len();
        let ncols: usize = y[0].len();
        // for i in 0..nrows {
        //     for j in 0..ncols {
        //         z[i][j] = dot(&x[i], &y[j]);
        //     }
        // }
        for j in 0..ncols {
            // matvecmul(x, &ytran[j], &mut z[j]);
            todo!()
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

    println!("{c:?}");
}
