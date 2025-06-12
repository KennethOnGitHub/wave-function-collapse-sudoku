use std::{array, collections::HashSet, fs};

type Board<T> = [[T;9];9];

type QuantumBoard = Board<QuantumCell>;
enum QuantumCell {
    Superposition(HashSet<u32>),
    Collapsed(u32)
}

#[derive(Clone , Copy)]
enum Cell {
    Filled(u32),
    Unfilled
}


fn main() {
    let sudoku_file = fs::read_to_string("sudoku.txt")
        .expect( "Sudoku File Not Read!");

    let initial_board: Board<Cell> = text_to_board(sudoku_file);

    print_board(&initial_board);

    // let unfilled_cells = initial_board
    //     .iter()
    //     .flatten()
    //     .filter(|&&x| matches!(x, Cell::Unfilled))
    //     .count();

    let mut superposition_board: QuantumBoard = array::from_fn(|_|
        array::from_fn(|_| QuantumCell::Superposition((0..=9).collect())
    ));

    for x in 0..9 {
        for y in 0..9{
            match superposition_board[x][y] {
                QuantumCell::Superposition(_) => {},
                QuantumCell::Collapsed(n) => {
                    for i in 0..9 {
                        match &mut superposition_board[i][y] {
                            QuantumCell::Collapsed(_) => {},
                            QuantumCell::Superposition(s) => {s.remove(&n);}
                        }

                        match &mut superposition_board[x][i] {
                            QuantumCell::Collapsed(_) => {},
                            QuantumCell::Superposition(s) => {s.remove(&n);}
                        }
                    }
                }
            }
        }
    }
}
    
fn text_to_board(board_string: String) -> Board<Cell>{
    let mut board: Board<Cell> = [[Cell::Unfilled; 9]; 9];

    let board_chars = board_string
        .chars()
        .filter(|&c| c.is_digit(10) || c == '_');

    let mut i = 0;
    for c in board_chars {
        let x = i % 9;
        let y = i / 9; 

        board[x][y] = match c.to_digit(10) {
            Some(n) => Cell::Filled(n),
            None if c == '_' => Cell::Unfilled,
            None => panic!("Invalid char in sudoku file!")
        };
        
        i += 1;
    }

    board
}

fn print_board(board: &Board<Cell>) {
    println!("-------------------------------");
    for y in 0..9 {

        print!("|");
        for x in 0..9 {
            match board[x][y] {
                Cell::Unfilled => {print!("   ")},
                Cell::Filled(num) => {print!(" {num} ")}
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
