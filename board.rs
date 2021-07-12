use ncurses::mvprintw;

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 35;

pub type PPBoard = [[char; WIDTH - 1]; HEIGHT - 1];


pub fn build_border(mat: &mut PPBoard) {
    for x in 0..mat[0].len() {
        mat[0][x] = '#';
        mat[mat.len() - 1][x] = '#';
    }
    for y in 1..mat.len() - 1 {
        mat[y][0] = '#';
        mat[y][(WIDTH / 2) as usize - 1] = '.';
        mat[y][(WIDTH / 2) as usize] = '.';
        mat[y][mat[0].len() - 1] = '#';
    }
}

pub fn build_scoreboard(mat: &mut PPBoard) {
    
}

pub fn draw_frame(mat: &PPBoard, wx: i32, wy: i32) {
    for y in 0..HEIGHT - 1 {
        for x in 0..WIDTH - 1 {
            mvprintw(y as i32 + wy, x as i32 + wx, String::from(mat[y][x]).as_str());
        }
    }
}

pub fn clean_frame(width: i32, height: i32) {
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            mvprintw(y, x, " ");
        }
    }
}