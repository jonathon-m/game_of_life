use rand::Rng;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, color, style};
use std::io::{Read, Write, stdout, stdin};
use std::{thread, time, process};
use termion::input::TermRead;
use termion::event::Key;

fn main() {

    thread::spawn(|| {
        game_of_life()
    });

    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => process::exit(0x0100),
            _ => {}
        }
    }

    
    
}

fn game_of_life() {

    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let size = (80, 40);
    let active_starting = 800;

    let mut grid = vec![vec![false; size.0];  size.1];

    for n in 0..active_starting {
        let x = rand::thread_rng().gen_range(1, size.1-1);
        let y = rand::thread_rng().gen_range(1, size.0-1);
        grid[x][y] = true;
    }   

    stdout.flush().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();

    while true {

        grid = set_next_grid(&grid);


        let grid_string = get_grid_string(&grid);
        
        stdout.flush().unwrap();

        write!(stdout, "{}", termion::cursor::Goto(10, 0)).unwrap();
        
        write!(stdout, "{}", grid_string).unwrap();

        let ten_millis = time::Duration::from_millis(100);
        let now = time::Instant::now();
        thread::sleep(ten_millis);

    }

}

fn get_grid_string(grid: &Vec<Vec<bool>>) -> String {
    let mut grid_string = String::new(); 
    grid_string.push('\n');
    grid_string.push('\r');
    for i in 1 .. grid.len() - 1 {
        grid_string.push('|');
        for j in 1 .. grid.get(0).unwrap().len() - 1 {
            if grid[i][j] {
                grid_string.push('@');
            } else {
                grid_string.push(' ');
            }
        }   
        grid_string.push('|');
        grid_string.push('\n');
        grid_string.push('\r');
    }

    return grid_string
}

fn set_next_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {

    let mut new_grid = vec![vec![false; grid.get(0).unwrap().len()];  grid.len()];

    for i in 1 .. grid.len() - 1 {
        for j in 1 .. grid.get(0).unwrap().len() - 1 {
            new_grid[i][j] = set_new_tile_value(i, j, &grid);
        }   
    }

    return new_grid;
}

fn set_new_tile_value(i: usize, j: usize, grid: &Vec<Vec<bool>>) -> bool {

    let neighbours = vec![
        (i-1, j-1), (i-1, j), (i-1, j+1),
        (i, j-1),             (i, j+1),
        (i+1, j-1), (i+1, j), (i+1, j+1)];

    let neighbours_iter = neighbours.iter();

    let mut live_cell_count = 0;

    for n in neighbours_iter {
        let neighbour_value = &grid[n.0][n.1];
        if *neighbour_value {
            live_cell_count += 1;
        }
    }

    let current_tile = &grid[i][j];

    if *current_tile && (live_cell_count == 2 || live_cell_count == 3) {
        return true;
        
    } else if !*current_tile && live_cell_count == 3 {
        return true;
    } else {
        return false;
    }
    

}

