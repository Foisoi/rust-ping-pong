use ncurses::mvprintw;
use std::ops::BitXor;
use std::time::{SystemTime, UNIX_EPOCH};

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 35;

pub type PPBoard = [[char; WIDTH - 1]; HEIGHT - 1];
#[derive(Copy, Clone)]
pub struct Vec2 { pub(crate) x: i32, pub(crate) y: i32 }

fn rn4() -> u8 {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let p = *(&t).to_string().as_bytes().last().unwrap();
    (t.bitxor(p as u128) as u8) % 4u8
}

pub fn random_trajectory() -> i8 {
    let mut sign = 1i8;
    if rn4() >= 2 {
        sign = -1;
    }
    sign
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
    pub fn random() -> Self {
        Vec2 {
            x: random_trajectory() as i32,
            y: random_trajectory() as i32,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PPBall {
    pos: Vec2,
    target: Vec2,
}

impl PPBall {
    pub fn new(x: i32, y: i32) -> Self {
        PPBall {
            pos: Vec2::new(x, y),
            target: Vec2::random(),
        }
    }

    pub fn get_pos(&self) -> Vec2 { self.pos }

    pub fn render(&mut self, mat: &mut PPBoard) {
        mat[self.pos.y as usize][self.pos.x as usize] = '0';
    }

    pub fn update_position_and_check_if_touches_border(&mut self, mat: &PPBoard) -> Option<u8> {
        let x = self.pos.x;
        let y = self.pos.y;
        let nx = x + self.target.x;
        let ny = y + self.target.y;

        if nx < 1 || ny < 1 || nx > (WIDTH - 3) as i32 || ny > (HEIGHT - 3) as i32 {
            if x == 1 || x == (WIDTH - 3) as i32 {
                self.target = Vec2::random();
                self.pos = Vec2::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
                return Some((x > (WIDTH / 2) as i32) as u8);
            }
            if y == 1 || y == (HEIGHT - 3) as i32 {
                self.target.y = -self.target.y;
            }
        } else if mat[ny as usize][nx as usize] == '|' {
            self.target.x = -self.target.x;
        }
        self.pos = Vec2::new(nx, ny);
        None
    }
}

#[derive(Copy, Clone)]
pub struct PPPlate {
    pos: Vec2,
}

impl PPPlate {
    pub fn new(x: i32, y: i32) -> Self {
        PPPlate { pos: Vec2::new(x, y) }
    }

    pub fn get_pos(&self) -> Vec2 { self.pos }

    pub fn step(&mut self, by: i32) {
        if self.pos.y + by > 4 && self.pos.y + by < (HEIGHT - 5) as i32 {
            self.pos.y += by;
        }
    }

    pub fn render(&mut self, mat: &mut PPBoard) {
        for p in (self.pos.y-4)..(self.pos.y+4) {
            mat[p as usize][self.pos.x as usize] = '|';
        }
    }
}

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