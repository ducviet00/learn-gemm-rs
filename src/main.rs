use std::time::Instant;

use rand::Rng;

const N: usize = 1024;
const BLOCK: usize = 32;

fn initialize_matrix() -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; N]; N];
    for x in 0..N {
        rand::thread_rng().fill::<[_]>(&mut matrix[x]);
    }
    matrix
}

fn naive_mm(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut matrix_c: Vec<Vec<f64>> = vec![vec![0.0; N]; N];
    for x in 0..N {
        for y in 0..N {
            let mut cxy: f64 = 0.0;
            for z in 0..N {
                cxy += matrix_a[x][z] * matrix_b[z][y];
            }
            matrix_c[x][y] = cxy;
        }
    }
    matrix_c
}

fn blocked_mm(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    assert!(N % BLOCK == 0);
    let mut matrix_c: Vec<Vec<f64>> = vec![vec![0.0; N]; N];
    for bx in (0..N).step_by(BLOCK) {
        for by in (0..N).step_by(BLOCK) {
            for x in bx..(bx+BLOCK) {
                for y in by..(by+BLOCK) {
                    let mut cxy: f64 = 0.0;
                    for z in 0..N {
                        cxy += matrix_a[x][z] * matrix_b[z][y];
                    }
                    matrix_c[x][y] = cxy;
                }
            }
        }
    }
    matrix_c
}

fn main() {
    
    let matrix_a: Vec<Vec<f64>> = initialize_matrix();
    let matrix_b: Vec<Vec<f64>> = initialize_matrix();

    let gflop: f32 = 2.0*((N*N*N) as f32)*10.0_f32.powi(-9);
    let instant = Instant::now();
    let _naive = naive_mm(&matrix_a, &matrix_b);
    println!("Naive MM {:?} GFLOP/S ", gflop as f32 / instant.elapsed().as_secs_f32());

    let instant = Instant::now();
    let _blocked = blocked_mm(&matrix_a, &matrix_b);
    println!("Blocked MM {:?} GFLOP/S ", gflop as f32 / instant.elapsed().as_secs_f32());

}
