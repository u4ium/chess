use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::board::{piece::Colour, BoardState};

pub trait Display {
    fn get_unique_id(&self) -> u32;
    fn display_board(&self, board_state: &BoardState);
    fn display_checkmate(&self, winner: Colour);
}

impl Hash for Box<dyn Display> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.get_unique_id().hash(state)
    }
}

impl PartialEq for Box<dyn Display> {
    fn eq(&self, other: &Box<dyn Display>) -> bool {
        self.get_unique_id() == other.get_unique_id()
    }
}

impl Eq for Box<dyn Display> {}

pub struct Displays(HashSet<Box<dyn Display>>);

impl Displays {
    pub fn new(displays: Vec<Box<dyn Display>>) -> Self {
        Displays(displays.into_iter().collect())
    }

    pub fn display_board(&self, board_state: &BoardState) {
        for Display in self.0.iter() {
            Display.display_board(board_state);
        }
    }

    pub fn display_checkmate(&self, winner: Colour) {
        for Display in self.0.iter() {
            Display.display_checkmate(winner);
        }
    }
}
