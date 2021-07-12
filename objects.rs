use crate::vec2::*;
use crate::board::*;


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