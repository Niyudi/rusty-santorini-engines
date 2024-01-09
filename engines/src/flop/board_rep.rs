use crate::models::Square;
use crate::models::Worker;
use crate::helpers::Turn;
use std::collections::HashSet;


pub struct Move {
    // From is the worker index 
    pub from: usize,
    pub to: usize,
    pub build:usize,
}

pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [usize; 4],
    pub turn: Turn,
}

#[derive(Debug, PartialEq)]
enum MoveError {
    InvalidFromSquare,
    InvalidToSquare,
    InvalidBuildSquare,
    OccupiedToSquare,
    OccupiedBuildSquare,
    HeightDifferenceHigh,
}

impl Board {
    fn square_is_free(&self, square:usize) -> bool {
        self.workers[0] != square && self.workers[1] != square && self.workers[2] != square && self.workers[3] != square &&
        self.blocks[square] < 4
    }

    fn move_is_legal(&self, mv: Move) -> Result<(), MoveError> {
        if mv.from != 0 && mv.from != 1 && mv.from != 2 && mv.from != 3 {
            return Err(MoveError::InvalidFromSquare);
        }
    
        if mv.to > 24 {
            return Err(MoveError::InvalidToSquare);
        }

        if mv.to > 24 {
            return Err(MoveError::InvalidBuildSquare);
        }

        if !self.square_is_free(mv.to){
            return Err(MoveError::OccupiedToSquare)
        }

        if !self.square_is_free(mv.build){
            return Err(MoveError::OccupiedBuildSquare)
        }

        if (self.blocks[mv.to] - self.blocks[mv.from]) > 1{
            return Err(MoveError::HeightDifferenceHigh)
        }
    
        Ok(())
    }

    fn make_move(&mut self, mv: Move){
        let sq = self.workers[mv.from];
        self.workers[mv.from] = mv.to;
        self.blocks[mv.build] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_from_square() {
        let mut board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 1, 0, 0, 0,
                     0, 4, 0, 2, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [13, 18, 7, 12],
            turn: Turn::W,
        };

        let invalid_move = Move { from: 25, to: 19, build: 14 };
        assert_eq!(board.move_is_legal(invalid_move), Err(MoveError::InvalidFromSquare));
    }
}