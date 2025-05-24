use std::{fs};

fn main() {
    let sudoku_file = fs::read_to_string("sudoku.txt")
        .expect("Sudoku File Not Read!");

    let initial_board: [[Option<u32> ; 9]; 9 ] = text_to_board(sudoku_file);

    print_board(&initial_board);
}
    
fn text_to_board(board_string: String) -> [[Option<u32> ; 9]; 9 ]{
    let mut board: [[Option<u32> ; 9]; 9 ] = [[None; 9]; 9];

    let board_chars = board_string
        .chars()
        .filter(|&c| c != '-' || c != '|' || c != ' ');

    let (mut i, mut j) = (0, 0);
    for c in board_chars {
        println!("{c}");
        if c.is_digit(10) {
            board[i][j] = c.to_digit(10);
            i +=1;
        }else if c == '_' {
            i += 1;
        }else if c == '\n' {
            j += 1;
            i = 0;
        }else {
            panic!("Sudoku File Has Invalid Char")
        }
    }

    return board;
}

fn print_board(board: &[[Option<u32>; 9]; 9 ]) {
    println!("-------------------------------");
    for y in 0..9 {

        print!("|");
        for x in 0..9 {
            match board[x][y] {
                Option::None => {print!("   ")},
                Option::Some(num) => {print!(" {num} ")}
            };
            if (x + 1) % 3 == 0 {
                print!("|");
            }
        }
        println!("");
        if (y + 1) % 3 == 0 {
            println!("-------------------------------");
        }
    }
}

// fn inverse_number_row(board: &[[Option<u32>; 9]; 9], y: usize) -> HashSet<i32> {
//     let mut possibleNumbers: HashSet<i32> = HashSet::from([1; 9]);

//     for i in 0..8 {
//         match board[i][y] {
//             Option::None => {},
//             Option::Some(cell) => {possibleNumbers.remove(&cell);}
//         };
//     }

//     return possibleNumbers;
// }
