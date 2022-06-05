pub fn _valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut table = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for row in board.iter() {
        for z in 1..10 {
            table[z] = 0;
        }
        for ch in row {
            match ch.to_digit(10) {
                None => continue,
                Some(idx) => {
                    if table[idx as usize] > 0 {
                        return false;
                    } else {
                        table[idx as usize] = 1
                    }
                }
            }
        }
    }
    for i in 0..9 {
        for z in 1..10 {
            table[z] = 0;
        }
        for row in board.iter() {
            match row[i].to_digit(10) {
                None => continue,
                Some(idx) => {
                    if table[idx as usize] > 0 {
                        return false;
                    } else {
                        table[idx as usize] = 1
                    }
                }
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            for z in 1..10 {
                table[z] = 0;
            }
            for row in 3 * i..3 * (i + 1) {
                for column in 3 * j..3 * (j + 1) {
                    match board[row][column].to_digit(10) {
                        None => continue,
                        Some(idx) => {
                            if table[idx as usize] > 0 {
                                return false;
                            } else {
                                table[idx as usize] = 1
                            }
                        }
                    }
                }
            }
        }
    }
    true
}
