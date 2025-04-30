use std::error::Error;

fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>> {
    let n = a.len();

    for i in 0..n {
        let mut max_row = i;
        for j in (i + 1)..n {
            if a[j][i].abs() > a[max_row][i].abs() {
                max_row = j;
            }
        }
        if a[max_row][i].abs() < 1e-12 {
            return Err("解けへんよ".into());
        }

        a.swap(i, max_row);
        b.swap(i, max_row);

        for j in (i + 1)..n {
            let factor = a[j][i] / a[i][i];
            for k in i..n {
                a[j][k] -= factor * a[i][k];
            }
            b[j] -= factor * b[i];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut tmp = b[i];
        for j in (i + 1)..n {
            tmp -= a[i][j] * x[j];
        }
        x[i] = tmp / a[i][i];
    }

    Ok(x)
}

pub fn lsm(data: &[(f64, f64)], degree: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    let m = degree + 1;
    let mut x = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            // x[i][j] = data.iter().map(|(x, _)| x.powi((degree * 2 - (i + j)) as i32)).sum();
            x[i][j] = data.iter().map(|(x, _)| x.powi((i + j) as i32)).sum();
        }
    }

    let mut y = vec![0.0; m];
    for i in 0..m {
        // y[i] = data.iter().map(|(x, y)| x.powi((degree - i) as i32) * y).sum();
        y[i] = data.iter().map(|(x, y)| x.powi(i as i32) * y).sum();
    }

    solve(x,y)
}

fn predict(x: f64, c: &[f64]) -> f64 {
    c.iter().enumerate().map(|(i, &a)| a * x.powi(i as i32)).sum()
}

pub fn predict_all(xs: &[f64], c: &[f64]) -> Vec<f64> {
    xs.iter().map(|&x| predict(x, c)).collect()
}
