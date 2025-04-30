use std::error::Error;

pub fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>> {
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
