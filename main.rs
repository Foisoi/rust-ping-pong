mod vec2;
mod board;
mod objects;

extern crate ncurses;

use ncurses::*;
use board::*;
use std::process::exit;
use std::time::Instant;


fn main() {
    // Initializing ncurses
    initscr();
    raw();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Initializing frame stuff
    let mut ball_delay = Instant::now();
    let mut machine_plate_delay = Instant::now();

    // Initializing matrix
    let mut board = [[' '; WIDTH - 1]; HEIGHT - 1];
    // Initializing objects
    let mut score: [u8; 2] = [0, 0];
    let mut player_plate = objects::PPPlate::new(6, 19);
    let mut machine_plate = objects::PPPlate::new((WIDTH as i32) - 6, 19);
    let mut ball = objects::PPBall::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32);

    // Drawing frame
    loop {
        match getch() {
            259 => {
                player_plate.step(-1);
            },
            3 | 4 => { // Pattern for exiting program if user pressed ctrl+c or ctrl+d
                endwin();
                // todo: Pretty-print score at the of the game
                exit(0);
            },
            ERR => {},
            _ => {
                player_plate.step(1);
            }
        }
        // Rendering all objects
        build_border(&mut board);
        ball.render(&mut board);
        player_plate.render(&mut board);
        machine_plate.render(&mut board);
        // Displaying matrix in terminal
        clean_frame(COLS(), LINES());
        draw_frame(&mut board, (COLS() - (WIDTH as i32)) / 2, 4);
        build_scoreboard(COLS() / 2, 1, &score);
        refresh();
        // Update ball position roughly every 40ms
        if ball_delay.elapsed().as_millis() > 40 {
            let collision = ball.update_position_and_check_if_touches_border(&board);
            match collision {
                Some(c) => {
                    score[1 - (c as usize)] += 1;
                },
                None => {},
            }
            ball_delay = Instant::now();
        }

        let threshold = (40f32 * ((WIDTH as f32) / (ball.get_pos().x as f32))) as u128; // For so called "equality" between machine and player
        if machine_plate_delay.elapsed().as_millis() > threshold {
            if ball.get_pos().y < machine_plate.get_pos().y {
                machine_plate.step(-1);
            } else {
                machine_plate.step(1);
            }
            machine_plate_delay = Instant::now();
        }
        // Cleaning the whole board (remove the next line if you want to see all possible ball paths for particular board size)
        board = [[' '; WIDTH - 1]; HEIGHT - 1];
    }
}
