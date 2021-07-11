mod primitives;
extern crate ncurses;

use ncurses::*;
use primitives::*;
use std::process::exit;
use std::time::Instant;


fn main() {
    // Initializing ncurses
    initscr();
    raw();
    noecho();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Initializing frame stuff
    let mut ball_delay = Instant::now();
    let mut machine_plate_delay = Instant::now();

    // Initializing matrix
    let mut board = [[' '; WIDTH - 1]; HEIGHT - 1];
    // Initializing objects' characteristics
    let mut score: [u8; 2] = [0, 0];
    let mut player_plate = PPPlate::new(6, 19);
    let mut machine_plate = PPPlate::new((WIDTH as i32) - 6, 19);
    let mut ball = PPBall::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32);

    // Drawing frame
    loop {
        let terminal_bounds = (COLS(), LINES()); // Width & height of terminal
        match getch() {
            259 => {
                player_plate.step(-1);
            },
            3 | 4 => { // Pattern for exiting program if ctrl+c or ctrl+d pressed
                echo();
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
        // Drawing in terminal
        clean_frame(COLS(), LINES());
        draw_frame(&mut board, (terminal_bounds.0 - (WIDTH as i32)) / 2, (terminal_bounds.1 - (HEIGHT as i32)) / 2);
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

        if machine_plate_delay.elapsed().as_millis() > 40 {
            if ball.get_pos().y < machine_plate.get_pos().y {
                machine_plate.step(-1);
            } else {
                machine_plate.step(1);
            }
            machine_plate_delay = Instant::now();
        }
        // Cleaning the whole board
        board = [[' '; WIDTH - 1]; HEIGHT - 1];
    }
}
