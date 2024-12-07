use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub positions: [[IVec2; 4]; 4],
    pub index: usize,
    pub rotate: usize
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType, index: usize) -> Self {
        let positions = match tetromino_type {
            TetrominoType::I => [
                [IVec2::new(-1,1), IVec2::new(0,1), IVec2::new(1,1), IVec2::new(2,1)],
                [IVec2::new(0,3), IVec2::new(0,2), IVec2::new(0,1), IVec2::new(0,0)],
                [IVec2::new(1,2), IVec2::new(0,2), IVec2::new(-1,2), IVec2::new(-2,2)],
                [IVec2::new(0,0), IVec2::new(0,1), IVec2::new(0,2), IVec2::new(0,3)],

            ],
            TetrominoType::O => [
                [IVec2::new(0,0), IVec2::new(1,0), IVec2::new(1,1), IVec2::new(0,1)],
                [IVec2::new(0,1), IVec2::new(0,0), IVec2::new(1,0), IVec2::new(1,1)],
                [IVec2::new(1,1), IVec2::new(0,1), IVec2::new(0,0), IVec2::new(1,0)],
                [IVec2::new(1,0), IVec2::new(1,1), IVec2::new(0,1), IVec2::new(0,0)],
            ],
            TetrominoType::J => [
                [IVec2::new(0,0), IVec2::new(1,0), IVec2::new(1,1), IVec2::new(1,2)],
                [IVec2::new(0,1), IVec2::new(0,0), IVec2::new(1,0), IVec2::new(2,0)],
                [IVec2::new(1,2), IVec2::new(0,2), IVec2::new(0,1), IVec2::new(0,0)],
                [IVec2::new(2,0), IVec2::new(2,1), IVec2::new(1,1), IVec2::new(0,1)],

            ],
            TetrominoType::L => [
                [IVec2::new(0,2), IVec2::new(0,1), IVec2::new(0,0), IVec2::new(1,0)],
                [IVec2::new(2,1), IVec2::new(1,1), IVec2::new(0,1), IVec2::new(0,0)],
                [IVec2::new(1,0), IVec2::new(1,1), IVec2::new(1,2), IVec2::new(0,2)],
                [IVec2::new(0,0), IVec2::new(1,0), IVec2::new(2,0), IVec2::new(2,1)],

            ],
            TetrominoType::S => [
                [IVec2::new(1,1), IVec2::new(0,1), IVec2::new(0,0), IVec2::new(-1,0)],
                [IVec2::new(0,0), IVec2::new(0,1), IVec2::new(-1,1), IVec2::new(-1,2)],
                [IVec2::new(-1,0), IVec2::new(0,0), IVec2::new(0,1), IVec2::new(1,1)],
                [IVec2::new(-1,2), IVec2::new(-1,1), IVec2::new(0,1), IVec2::new(0,0)],

            ],
            TetrominoType::T => [
                [IVec2::new(0,1), IVec2::new(1,0), IVec2::new(0,0), IVec2::new(-1,0)],
                [IVec2::new(1,1), IVec2::new(0,0), IVec2::new(0,1), IVec2::new(0,2)],
                [IVec2::new(0,0), IVec2::new(-1,1), IVec2::new(0,1), IVec2::new(1,1)],
                [IVec2::new(-1,1), IVec2::new(0,2), IVec2::new(0,1), IVec2::new(0,0)],
            ],
            TetrominoType::Z => [
                [IVec2::new(-1,1), IVec2::new(0,1), IVec2::new(0,0), IVec2::new(1,0)],
                [IVec2::new(0,2), IVec2::new(0,1), IVec2::new(-1,1), IVec2::new(-1,0)],
                [IVec2::new(1,0), IVec2::new(0,0), IVec2::new(0,1), IVec2::new(-1,1)],
                [IVec2::new(-1,0), IVec2::new(-1,1), IVec2::new(0,1), IVec2::new(0,2)],
            ],
        };

        Self {
            tetromino_type,
            positions,
            index,
            rotate: 0
        }
    }

    pub fn get_position(&self) -> [IVec2; 4] {
        self.positions[self.rotate]
    }

    pub fn get_position2(&self, rotate: usize) -> [IVec2; 4] {
        self.positions[rotate]
    }

    pub fn rotate_left(&mut self) {
        self.rotate = (self.rotate + 3) % 4;
    }

    pub fn rotate_right(&mut self) {
        self.rotate = (self.rotate + 1) % 4;
    }
    pub fn down_most_position(&self) -> Vec<IVec2>{
        let position = self.get_position();

        match self.rotate {
            0 => match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[0], position[1]],
                    TetrominoType::J =>
                        vec![position[0], position[1]],
                    TetrominoType::L =>
                        vec![position[2], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2], position[3]],
                    TetrominoType::T =>
                        vec![position[1], position[2], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2], position[3]],
                }
            1 => match self.tetromino_type {
                    TetrominoType::I => vec![position[3]],
                    TetrominoType::O =>
                        vec![position[1], position[2]],
                    TetrominoType::J =>
                        vec![position[1], position[2], position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2]],
                    TetrominoType::T =>
                        vec![position[0], position[1]],
                    TetrominoType::Z =>
                        vec![position[1], position[3]],
                }
            2 => match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[2], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[1], position[3]],
                }
            3 => match self.tetromino_type {
                    TetrominoType::I => vec![position[0]],
                    TetrominoType::O =>
                        vec![position[0], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[2], position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[2]],
                    TetrominoType::S =>
                        vec![position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2]],
                },
            _ => panic!("Invalid rotate number!"),
        }

    }

    pub fn right_most_position(&self) -> Vec<IVec2> {
        let position = self.get_position();
        match self.rotate {
            0 =>
                match self.tetromino_type {
                    TetrominoType::I =>
                        vec![position[3]],
                    TetrominoType::O =>
                        vec![position[1], position[2]],
                    TetrominoType::J =>
                        vec![position[1], position[2],position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2]],
                    TetrominoType::T =>
                        vec![position[0], position[1]],
                    TetrominoType::Z =>
                        vec![position[1], position[3]],
                }
            1 =>
                match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[2], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[1], position[3]],
                }
            2 =>
                match self.tetromino_type {
                    TetrominoType::I =>
                        vec![position[0]],
                    TetrominoType::O =>
                        vec![position[0], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[2],position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[2]],
                    TetrominoType::S =>
                        vec![position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2]],
                }
            3 =>
                match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[0], position[1]],
                    TetrominoType::J =>
                        vec![position[0], position[1]],
                    TetrominoType::L =>
                        vec![position[2], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2], position[3]],
                    TetrominoType::T =>
                        vec![position[1], position[2], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2], position[3]],
                },
            _ => panic!("Invalid rotate number!")
        }
    }

    pub fn left_most_position(&self) -> Vec<IVec2> {
        let position = self.get_position();
        match self.rotate {
            0 =>
                match self.tetromino_type {
                    TetrominoType::I => vec![position[0]],
                    TetrominoType::O =>
                        vec![position[0], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[2],position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[2]],
                    TetrominoType::S =>
                        vec![position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2]],
                }
            1 =>
                match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[0], position[1]],
                    TetrominoType::J =>
                        vec![position[0], position[1]],
                    TetrominoType::L =>
                        vec![position[2], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2], position[3]],
                    TetrominoType::T =>
                        vec![position[1], position[2], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[2], position[3]],
                }
            2 =>
                match self.tetromino_type {
                    TetrominoType::I =>
                        vec![position[3]],
                    TetrominoType::O =>
                        vec![position[1], position[2]],
                    TetrominoType::J =>
                        vec![position[1], position[2],position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[2]],
                    TetrominoType::T =>
                        vec![position[0], position[1]],
                    TetrominoType::Z =>
                        vec![position[1], position[3]],
                }
            3 =>
                match self.tetromino_type {
                    TetrominoType::I => Vec::from(position),
                    TetrominoType::O =>
                        vec![position[2], position[3]],
                    TetrominoType::J =>
                        vec![position[0], position[3]],
                    TetrominoType::L =>
                        vec![position[0], position[3]],
                    TetrominoType::S =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::T =>
                        vec![position[0], position[1], position[3]],
                    TetrominoType::Z =>
                        vec![position[0], position[1], position[3]],
                },
            _ => panic!("Invalid rotate number!")
        }
    }


}