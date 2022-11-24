use rand::Rng;
use std::{thread, time};

const LIFE_PROB: u32 = 35; // initial state
const SLEEP_MSEC: u64 = 50;
const SPRINKLE: bool = true; // Revive random cells
const SPRINKLE_RATE: u32 = 400; // How often
const SPRINKLE_PROB: u32 = 5; // How many

fn main() {
    let (width, height) = term_size::dimensions().unwrap();
    let mut board = vec![false; width * height];
    let mut new_board = board.clone(); // new state can't be store in the same board

    activate_cells(&mut board, LIFE_PROB);

    let sleep_dur = time::Duration::from_millis(SLEEP_MSEC);
    let mut out = String::with_capacity(height * width);
    let mut sprinkle_iter = 0;

    loop {
        out.clear();
        next_gen(&board, &mut new_board, width);
        board = new_board.clone();

        if SPRINKLE {
            sprinkle_iter += 1;
            if sprinkle_iter == SPRINKLE_RATE {
                activate_cells(&mut board, SPRINKLE_PROB);
                sprinkle_iter = 0;
            }
        }

        // stringify board
        for cell in &board {
            if *cell {
                out.push('O')
            } else {
                out.push(' ')
            };
        }

        print!("\x1B[2J{}", out); // clear screen
        thread::sleep(sleep_dur);
    }
}

fn next_gen(board: &Vec<bool>, new_board: &mut Vec<bool>, width: usize) {
    let mut alive_neighboars: u8;
    for i in width + 2..board.len() - width - 1 {
        alive_neighboars = 0;
        for idx in [
            i - width - 1,
            i - width,
            i - width + 1,
            i - 1,
            i + 1,
            i + width - 1,
            i + width,
            i + width + 1,
        ] {
            alive_neighboars += board[idx] as u8
        }

        if board[i] {
            if alive_neighboars != 2 && alive_neighboars != 3 {
                new_board[i] = false;
            }
        } else if alive_neighboars == 3 {
            new_board[i] = true;
        }
    }
}

fn activate_cells(board: &mut Vec<bool>, prob: u32) {
    for cell in board {
        if rand::thread_rng().gen_range(1..=100) < prob {
            *cell = true;
        }
    }
}
