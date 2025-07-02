use std::{array, collections::HashSet, fs};

type Board<T> = [[T; 9]; 9];

#[derive(Clone, Copy)]
enum SudokuCell {
    Filled(u32),
    Unfilled,
}
type SudokuBoard = Board<SudokuCell>;

enum QuantumCell {
    Collapsed(u32),
    Superposition(HashSet<u32>),
}
struct QuantumBoard {
    board: Board<QuantumCell>,
}

impl QuantumBoard {
    fn create_from_board(starting_board: &SudokuBoard) -> QuantumBoard {
        let mut new_board = QuantumBoard {
            board: {
                array::from_fn(|_| {
                    array::from_fn(|_| QuantumCell::Superposition((1..=9).collect()))
                })
            },
        };

        for x in 0..9 {
            for y in 0..9 {
                match starting_board[x][y] {
                    SudokuCell::Filled(val) => {
                        new_board.collapse_cell(x, y, val);
                    }
                    SudokuCell::Unfilled => {}
                };
            }
        }

        new_board
    }

    fn remove_posibility(&mut self, x: usize, y: usize, v: u32) {
        match &mut self.board[x][y] {
            QuantumCell::Collapsed(_) => {}
            QuantumCell::Superposition(s) => {
                s.remove(&v);
            }
        }
    }

    fn collapse_cell(&mut self, x: usize, y: usize, val: u32) {

        fn propagate_collapse(board: &mut QuantumBoard, x: usize, y: usize, val: u32) {
            for i in 0..9 {
                board.remove_posibility(x, i, val);
                board.remove_posibility(y, x, val);
            }

            let nonadrant_x = x / 3;
            let nonadrant_y = y / 3;
            for i in 0..3 {
                for j in 0..3 {
                    board.remove_posibility(3 * nonadrant_x + i, 3 * nonadrant_y + j, val);
                }
            }
        }

        self.board[x][y] = match &mut self.board[x][y] {
            QuantumCell::Collapsed(_) => {
                panic!("Cell has already been collapsed!");
            }
            QuantumCell::Superposition(s) => QuantumCell::Collapsed(val),
        };

        propagate_collapse(self, x, y, val);


    }
}

fn main() {
    let sudoku_file = fs::read_to_string("sudoku.txt").expect("Sudoku File Not Read!");

    let initial_board: Board<SudokuCell> = text_to_board(sudoku_file);

    print_board(&initial_board);

    let superposition_board = QuantumBoard::create_from_board(&initial_board);

    // let finalboard = loop {
    //     let entropy_board: Board<usize> =
    //         superposition_board.board
    //             .map(|col: [QuantumCell; 9]|
    //                 col.map(|q_cell: QuantumCell| 0)
    //             );
    // };

    let entropy_board: Board<usize> = superposition_board.&board.map(|col: &[QuantumCell; 9]| {
        col.map(|q_cell: QuantumCell| match &q_cell {
            QuantumCell::Collapsed(_) => 0,
            QuantumCell::Superposition(s) => s.len(),
        })
    });

    println!("{}", entropy_board[0][2])
}

fn text_to_board(board_string: String) -> Board<SudokuCell> {
    let mut board: Board<SudokuCell> = [[SudokuCell::Unfilled; 9]; 9];

    let board_chars = board_string.chars().filter(|&c| c.is_digit(10) || c == '_');

    let mut i = 0;
    for c in board_chars {
        let x = i % 9;
        let y = i / 9;

        board[x][y] = match c.to_digit(10) {
            Some(n) => SudokuCell::Filled(n),
            None if c == '_' => SudokuCell::Unfilled,
            None => panic!("Invalid char in sudoku file!"),
        };

        i += 1;
    }

    board
}

fn print_board(board: &Board<SudokuCell>) {
    println!("-------------------------------");
    for y in 0..9 {
        print!("|");
        for x in 0..9 {
            match board[x][y] {
                SudokuCell::Unfilled => {
                    print!("   ")
                }
                SudokuCell::Filled(num) => {
                    print!(" {num} ")
                }
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
