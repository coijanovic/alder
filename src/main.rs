use rand::prelude::*;
use std::{thread, time};

const SIZE : usize = 20;
const CHANCE : f64 = 0.20;
const SLEEP_INTERVAL : time::Duration = time::Duration::from_millis(500);

/// Prints the field in my *custom aesthetic*
fn print_field(field: &[[bool; SIZE]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if field[i][j] {
                print!("🌝")
            } else {
                print!("🌚")
            }
        }
        print!("\n")
    }
}

/// Initializes the field with random values according to CHANCE const
fn init_random_field(field: &mut [[bool; SIZE]; SIZE]) {
    let mut rng = rand::thread_rng();
    for i in 0..SIZE {
        for j in 0..SIZE {
            let roll : f64 = rng.gen();
            if roll > CHANCE {
                field[i][j] = false;
            } else {
                field[i][j] = true;
            }
        }
    }
}

/// Returns how many living neighbors the point coord has
fn get_living_neighbor_count(coord: (i32, i32), field: &[[bool; SIZE]; SIZE]) -> u8 {
    let mut count: u8 = 0;
    for i in coord.0-1..coord.0+2 {
        for j in coord.1-1..coord.1+2 {
            if i < 0 || j < 0 || i >= SIZE as i32 || j >= SIZE as i32 {
                continue;
            }
            if (i == coord.0 && j == coord.1) || (i == coord.1 && j == coord.0) {
                // continue if (i,j) == coord
                continue;
            }
            if field[i as usize][j as usize] {
                count += 1;
            }
        }
    }
    return count;
}

/// Evolves the given field one tick
fn tick(field: &mut [[bool; SIZE]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            let lnc: u8 = get_living_neighbor_count((i as i32, j as i32), &field);
            if field[i][j] && (lnc == 2 || lnc == 3) {
                // Any live cell with two or three live neighbours survives.
                field[i][j] = true;
            } else if !field[i][j] && lnc == 3 {
                //Any dead cell with three live neighbours becomes a live cell.
                field[i][j] = true;
            } else {
                // All other live cells die in the next generation.
                // Similarly, all other dead cells stay dead.
                field[i][j] = false;
            }
        }
    }
}

fn main() {
    let mut field = [[true; SIZE]; SIZE];
    init_random_field(&mut field);
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print_field(&field);
        tick(&mut field);
        thread::sleep(SLEEP_INTERVAL);
    }
}
