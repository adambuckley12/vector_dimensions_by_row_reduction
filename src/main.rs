use std::ops::RangeInclusive;
use std::fs::File;
use rand::prelude::*;
use csv::Writer;
use std::error::Error;

const MIN_ALLOWED_MATRIX_VALUE: i64 = 0; // Adjust this to change the min value for range of the random numbers 
const MAX_ALLOWED_MATRIX_VALUE: i64 = 1000; // Adjust this to change the max value for range of the random number
const VECTOR_LENGTH: RangeInclusive<usize> = 1..=20; // Adjust this to change the range of the vector lengths
const VECTOR_COUNT: RangeInclusive<usize> = 1..=20; // Adjust this to change the range of the vector counts


fn main() -> Result<(), Box<dyn Error>> { 
    let mut output_data: Vec<Vec<String>> = Vec::new();

    for size in VECTOR_LENGTH {
        let mut average_ranks = Vec::new();
        for r in VECTOR_COUNT.clone() { 
            let average_rank: f64 = get_average_rank(r, size, 100);
            average_ranks.push(average_rank.to_string());
        }
        output_data.push(average_ranks);
    }

    // Transpose the data so that it is in the correct format for the CSV file
    let transposed_data: Vec<Vec<String>> = output_data
        .iter()
        .map(|row| row.iter().cloned().collect())
        .collect();

    // Create a CSV file and write the data
    let file: File = File::create("average_ranks_0.csv")?;
    let mut writer = Writer::from_writer(file);

    for row in transposed_data {
        writer.write_record(row)?;
    }

    print!("Finished writing to file\n");

    writer.flush()?;
    Ok(())
}

fn get_average_rank(size: usize, n_vectors: usize, trials: usize) -> f64 {
    let mut total: f64 = 0.0;

    //count the amount of times each rank is achieved
    let mut rank_count: Vec<i64> = vec![0; size as usize + 1];
    for _ in 0..trials {
        let mut mat = generate_random_matrix(size, n_vectors);
        row_reduce(&mut mat);
        let rank = count_non_empty_rows(&mat);
        total += rank as f64;
        rank_count[rank as usize] += 1;
    }

    //print the amount of times each rank is achieved only for when size = n_vectors
    if size == n_vectors {
        println!("Rank count for size = {}", size);
        for i in 0..size + 1 {
            // dont print if rank is 0
            if rank_count[i as usize] == 0 {
                continue;
            }
            println!("Rank {} achieved {} times", i, rank_count[i as usize]);
        }
    }
    total / trials as f64
}

fn generate_random_matrix(size: usize, n_vectors: usize) -> Vec<Vec<i64>> {
    let mut rng = thread_rng();
    let mut matrix = Vec::new();

    for _ in 0..n_vectors {
        let vector: Vec<i64> = (0..size).map(|_| rng.gen_range(MIN_ALLOWED_MATRIX_VALUE..=MAX_ALLOWED_MATRIX_VALUE)).collect(); // Adjust this to change the range of the random numbers (0..=1 means 0 or 1 inclusive)
        matrix.push(vector);
    }

    matrix
}

fn count_non_empty_rows(matrix: &Vec<Vec<i64>>) -> i64 {
    matrix.iter().filter(|row| row.iter().any(|&value| value != 0)).count() as i64
}

fn row_reduce(matrix: &mut Vec<Vec<i64>>) {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut pivot_col = 0;

    for current_row in 0..num_rows {
        if pivot_col >= num_cols {
            break;
        }

        let mut next_row = current_row;

        while matrix[next_row][pivot_col] == 0 {
            next_row += 1;
            if next_row == num_rows {
                next_row = current_row;
                pivot_col += 1;
                if pivot_col == num_cols {
                    return;
                }
            }
        }

        matrix.swap(current_row, next_row);

        let leading_value = matrix[current_row][pivot_col];

        // Scale current row to make element 1
        for col in 0..num_cols {
            matrix[current_row][col] /= leading_value;
        }

        // Make other rows 0 in the current column
        for other_row in 0..num_rows {
            if other_row != current_row {
                let factor = matrix[other_row][pivot_col];
                for col in 0..num_cols {
                    // Subtract scaled current row from other rows
                    matrix[other_row][col] -= factor * matrix[current_row][col];
                }
            }
        }

        pivot_col += 1;
    }
}
