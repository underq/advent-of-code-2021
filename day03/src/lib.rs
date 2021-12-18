pub fn step_one(matrix: Vec<Vec<char>>) -> i32 {
    let half_matrix_len: u32 = (matrix.len() / 2) as u32;
    let bits_size = matrix[0].len();
    let mut sum = vec![0; bits_size];

    for bits in matrix {
        let mut i = 0;
        for bit in bits {
            sum[i] += bit.to_digit(10).unwrap();
            i += 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for total in sum {
        if total > half_matrix_len {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2;
        } else {
            gamma = gamma * 2;
            epsilon = epsilon * 2 + 1;
        }
    }

    return gamma * epsilon;
}


pub fn step_two(matrix: Vec<Vec<char>>) -> i32 {
    let half_matrix_len: u32 = (matrix.len() / 2) as u32;
    let bits_size = matrix[0].len();
    let mut sum = vec![0; bits_size];

    for bits in matrix {
        let mut i = 0;
        for bit in bits {
            sum[i] += bit.to_digit(10).unwrap();
            i += 1;
        }
    }

    matrix
        .iter()
        .filter(|(v)| );

    let mut gamma = 0;
    let mut epsilon = 0;

    for total in sum {
        if total > half_matrix_len {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2;
        } else {
            gamma = gamma * 2;
            epsilon = epsilon * 2 + 1;
        }
    }

    return gamma * epsilon;
}