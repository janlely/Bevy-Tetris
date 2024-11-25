use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub positions: [[UVec2; 4]; 4],
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        let positions = match tetromino_type {
            TetrominoType::I => [
                [Vec2::new(0, 0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0), Vec2::new(0.0, 3.0)],
                [Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0)],
                [Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 2.0), Vec2::new(1.0, 3.0)],
                [Vec2::new(-1.0, 2.0), Vec2::new(0.0, 2.0), Vec2::new(1.0, 2.0), Vec2::new(2.0, 2.0)],
            ],
            TetrominoType::J => [
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0), Vec2::new(-1.0, 2.0)],
                [Vec2::new(-1.0, 0.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0)],
                [Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 2.0)],
            ],
            TetrominoType::L => [
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0), Vec2::new(1.0, 2.0)],
                [Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 2.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(-1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0)],
                [Vec2::new(1.0, 0.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
            ],
            TetrominoType::O => [
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
            ],
            TetrominoType::S => [
                [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(-1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 2.0)],
                [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0)],
                [Vec2::new(-1.0, 0.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0)],
            ],
            TetrominoType::T => [
                [Vec2::new(0.0, 0.0), Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 2.0)],
            ],
            TetrominoType::Z => [
                [Vec2::new(0.0, 0.0), Vec2::new(-1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 2.0)],
                [Vec2::new(-1.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(-1.0, 1.0), Vec2::new(-1.0, 2.0)],
            ],
        };

        Self {
            tetromino_type,
            positions,
            color,
        }
    }
}