use std::ops::Deref;

use sdl2::pixels::Color;

use crate::rotations::get_rotations;

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 22;
pub const FIELD_VIS_HEIGHT: usize = 20;

#[derive(Clone, Copy)]
pub enum RotationState {
    None,
    Right,
    Flip,
    Left,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    Z,
    S,
    T,
}

impl Into<Color> for PieceType {
    fn into(self) -> Color {
        match self {
            PieceType::I => Color::RGB(0, 255, 255),
            PieceType::J => Color::BLUE,
            PieceType::L => Color::RGB(255, 128, 0),
            PieceType::O => Color::YELLOW,
            PieceType::Z => Color::RED,
            PieceType::S => Color::GREEN,
            PieceType::T => Color::RGB(127, 0, 255),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    x: usize,
    y: usize,
    typ: PieceType,
    rot: RotationState,
}

#[derive(Clone, Copy)]
pub struct Cell {
    typ: Option<PieceType>,
}

#[derive(Clone, Copy)]
pub struct Row {
    cells: [Cell; 10],
}

#[derive(Clone, Copy)]
pub struct Field {
    rows: [Row; 22],
    piece: Piece,
}

impl Field {
    pub fn new() -> Self {
        Field {
            piece: Piece {
                x: 0,
                y: 6,
                typ: PieceType::I,
                rot: RotationState::None,
            },
            rows: [Row {
                cells: [Cell { typ: None }; 10],
            }; 22],
        }
    }

    fn try_place_piece(&mut self, x: usize, y: usize, typ: PieceType, rot: RotationState) -> bool {
        for j in 0..4 {
            for i in 0..4 {
                if x + i >= FIELD_WIDTH
                    || y + j >= FIELD_HEIGHT
                    || (get_rotations(typ, rot)[j][i]
                        && self.rows[y + j].cells[x + i].typ.is_some())
                {
                    return false;
                }
            }
        }

        self.piece = Piece { x, y, typ, rot };

        true
    }

    pub fn get_cell_color(&self, x: usize, y: usize) -> Color {
        match self.rows[y].cells[x].typ {
            Some(t) => t.into(),
            None => {
                if x >= self.piece.x
                    && x < self.piece.x + 4
                    && y >= self.piece.y
                    && y < self.piece.y + 4
                    && get_rotations(self.piece.typ, self.piece.rot)[y - self.piece.y]
                        [x - self.piece.x]
                {
                    self.piece.typ.into()
                } else {
                    Color::BLACK
                }
            }
        }
    }

    pub fn piece_up(&mut self) {
        self.try_place_piece(
            self.piece.x,
            self.piece.y - 1,
            self.piece.typ,
            self.piece.rot,
        );
    }

    pub fn piece_down(&mut self) {
        self.try_place_piece(
            self.piece.x,
            self.piece.y + 1,
            self.piece.typ,
            self.piece.rot,
        );
    }

    pub fn piece_left(&mut self) {
        self.try_place_piece(
            self.piece.x.saturating_sub(1),
            self.piece.y,
            self.piece.typ,
            self.piece.rot,
        );
    }
    pub fn piece_right(&mut self) {
        self.try_place_piece(
            self.piece.x + 1,
            self.piece.y,
            self.piece.typ,
            self.piece.rot,
        );
    }
}
