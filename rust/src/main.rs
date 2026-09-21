// use std::ops::{AddAssign, Mul};

// fn dot<T: Mul + AddAssign>(x: Vec<T>, y: Vec<T>) {
fn dot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    // assert_eq!(x.len(), y.len())
    let n: usize = x.len();
    let mut acc: f64 = 0.0;
    for i in 0..n {
        acc += x[i] * y[i];
    }
    return acc;
}

fn matmul(x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>, z: &mut Vec<Vec<f64>>) {
    let nrows: usize = x.len();
    let ncols: usize = y.len();
    for i in 0..nrows {
        for j in 0..ncols {
            z[i][j] = dot(&x[i], &y[j]);
        }
    }
}

fn main() {
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let y: Vec<f64> = vec![1.0, 2.0, 3.0];
    println!("{x:?}.{y:?} = {}", dot(&x, &y));

    let a: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];

    let b: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let mut c: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];
    matmul(&a, &b, &mut c);

    println!("{c:?}");
}
