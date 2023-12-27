use std::fs::read_to_string;
//use std::collections::{BTreeSet,HashMap};
//use regex::Regex;

use num_integer::Integer;
use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = 200_000_000_000_000..=400_000_000_000_000;


pub fn part2(input: &[[i64; 6]]) -> i128 {
    // Calculations need the range of `i128`.
    let widen = |i: usize| input[i].map(|n| n as i128);
    let [a, b, c, d, e, f] = widen(0);
    let [g, h, i, j, k, l] = widen(1);
    let [m, n, o, p, q, r] = widen(2);

    // Coefficients for the 6 simulataneous linear equations.
    // Columns are px, py, pz, vx, vy, vz of the rock equal to a constant.
    let mut matrix = [
        [0, l - f, e - k, 0, c - i, h - b, e * c - b * f + h * l - k * i],
        [0, r - f, e - q, 0, c - o, n - b, e * c - b * f + n * r - q * o],
        [f - l, 0, j - d, i - c, 0, a - g, a * f - d * c + j * i - g * l],
        [f - r, 0, p - d, o - c, 0, a - m, a * f - d * c + p * o - m * r],
        [k - e, d - j, 0, b - h, g - a, 0, d * b - a * e + g * k - j * h],
        [q - e, d - p, 0, b - n, m - a, 0, d * b - a * e + m * q - p * n],
    ];

    // Use Gaussian elimination to solve for the 6 unknowns.
    // Forward elimination, processing columns from left to right.
    // This will leave a matrix in row echelon form.
    for pivot in 0..6 {
        // Make leading coefficient of each row positive to make subsequent calculations easier.
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                // Flip signs of each coefficient.
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            // Reduce by GCD each time otherwise coefficients will overflow even a `i128`.
            for row in &mut matrix[pivot..] {
                let mut factor = 0;

                for &next in &row[pivot..] {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.abs();
                        } else {
                            factor = factor.gcd(&next.abs());
                        }
                    }
                }

                row[pivot..].iter_mut().for_each(|c| *c /= factor);
            }

            let column = matrix.map(|row| row[pivot]);

            // If only one non-zero coefficient remaining in the column then we're done.
            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                // Move this row into the pivot location
                let index = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            // Find the row with the lowest non-zero leading coefficient.
            let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index = column.iter().rposition(|&c| c == min).unwrap();

            // Subtract as many multiples of this minimum row from each other row as possible
            // to shrink the coefficients of our column towards zero.
            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor = column[row] / min;

                    for col in pivot..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    // Back substitution, processing columns from right to left.
    // This will leave the matrix in reduced row echelon form.
    // The solved unknowns are then in the 7th column.
    for pivot in (0..6).rev() {
        // We're explicitly told that the results are integers so integer division is safe
        // and will not mangle result.
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    // x + y + z
    matrix[0][6] + matrix[1][6] + matrix[2][6]
}

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}



fn main(){
    //let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^(\d+), (\d+), (\d+) @ (\d+), (\d+), (\d+)$").unwrap();
    
    let mut hails = Vec::new();
    for line in text.iter() {
        let mut a = line.split(" @ ");
        let mut b = a.next().unwrap().split(", ").map(|s| s.trim().parse::<i64>().
        unwrap());
        let mut c = a.next().unwrap().split(", ").map(|s| s.trim().parse::<i64>().
        unwrap());
        hails.push([b.next().unwrap(), b.next().unwrap(), b.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap()]);
    }
    
    println!("{}",part2(&hails))
}

// A+tU
// B+sV

// t = (Vx(Ay-By)-Vy(Ax-Bx))/(UxVy-UyVx)