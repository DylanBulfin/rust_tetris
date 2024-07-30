use rand::random;
use sdl2::pixels::Color;

use crate::rotations::{get_coords, get_wallkicks};

pub const FIELD_WIDTH: usize = 12;
pub const FIELD_VIS_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 22;
pub const FIELD_VIS_HEIGHT: usize = 20;

#[derive(Clone, Copy)]
pub enum RotationState {
    None,
    Right,
    Flip,
    Left,
}

impl RotationState {
    pub fn right(&self) -> Self {
        match self {
            RotationState::None => RotationState::Right,
            RotationState::Right => RotationState::Flip,
            RotationState::Flip => RotationState::Left,
            RotationState::Left => RotationState::None,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            RotationState::None => RotationState::Left,
            RotationState::Left => RotationState::Flip,
            RotationState::Flip => RotationState::Right,
            RotationState::Right => RotationState::None,
        }
    }
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

impl From<u32> for PieceType {
    fn from(value: u32) -> Self {
        match value {
            0 => PieceType::I,
            1 => PieceType::J,
            2 => PieceType::L,
            3 => PieceType::O,
            4 => PieceType::Z,
            5 => PieceType::S,
            6 => PieceType::T,
            _ => panic!("Only works if value in range [0..7)"),
        }
    }
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
pub struct Ghost {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
pub struct Cell {
    typ: Option<PieceType>,
}

#[derive(Clone, Copy)]
pub struct Row {
    cells: [Cell; 12],
}

#[derive(Clone, Copy)]
pub struct Field {
    rows: [Row; 22],
    piece: Piece,
    ghost: Ghost,
}

impl Field {
    pub fn new() -> Self {
        Field {
            piece: Piece {
                x: 5,
                y: 5,
                typ: PieceType::I,
                rot: RotationState::None,
            },
            ghost: Ghost { x: 0, y: 0 },
            rows: [Row {
                cells: [Cell { typ: None }; 12],
            }; 22],
        }
    }

    pub fn next_piece(&mut self) -> bool {
        //TODO change how the piece is generated
        let typ = PieceType::from(random::<u32>() % 7);

        self.try_place_piece(Piece {
            x: 5,
            y: 5,
            typ,
            rot: RotationState::None,
        })
    }

    pub fn spawn_piece(&mut self, typ: PieceType) -> bool {
        println!("Remove this later");
        self.try_place_piece(Piece {
            x: 5,
            y: 5,
            typ,
            rot: RotationState::None,
        })
    }

    pub fn drop(&mut self, hardly: bool) {
        while self.try_place_piece(Piece {
            y: self.piece.y + 1,
            ..self.piece
        }) {}

        if hardly {
            self.lock_piece();
        }
    }

    pub fn lock_piece(&mut self) -> bool {
        for (r, c) in get_coords(self.piece.typ, self.piece.rot) {
            self.rows[self.piece.y + r].cells[self.piece.x + c].typ = Some(self.piece.typ)
        }

        self.next_piece()
    }

    fn can_place_piece(&mut self, pc: Piece) -> bool {
        for (r, c) in get_coords(pc.typ, pc.rot) {
            if pc.x + c >= FIELD_WIDTH
                || pc.y + r >= FIELD_HEIGHT
                || pc.x + c < 2
                || self.rows[pc.y + r].cells[pc.x + c].typ.is_some()
            {
                return false;
            }
        }

        true
    }

    fn update_ghost(&mut self) {
        let mut piece = self.piece;

        loop {
            if self.can_place_piece(piece) {
                self.ghost.x = piece.x;
                self.ghost.y = piece.y;
                piece = Piece {
                    y: piece.y + 1,
                    ..piece
                }
            } else {
                break;
            }
        }
    }

    fn try_place_piece(&mut self, pc: Piece) -> bool {
        if self.can_place_piece(pc) {
            self.piece = pc;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn get_cell_color(&self, x: usize, y: usize) -> Color {
        match self.rows[y].cells[x].typ {
            Some(t) => t.into(),
            None => {
                if x >= self.piece.x
                    && y >= self.piece.y
                    && get_coords(self.piece.typ, self.piece.rot)
                        .contains(&(y - self.piece.y, x - self.piece.x))
                {
                    self.piece.typ.into()
                } else if x >= self.ghost.x
                    && y >= self.ghost.y
                    && get_coords(self.piece.typ, self.piece.rot)
                        .contains(&(y - self.ghost.y, x - self.ghost.x))
                {
                    Color::WHITE
                } else {
                    Color::BLACK
                }
            }
        }
    }

    pub fn piece_up(&mut self) {
        self.try_place_piece(Piece {
            y: self.piece.y.saturating_sub(1),
            ..self.piece
        });
    }

    pub fn piece_down(&mut self) {
        self.try_place_piece(Piece {
            y: self.piece.y + 1,
            ..self.piece
        });
    }

    pub fn piece_left(&mut self) {
        self.try_place_piece(Piece {
            x: self.piece.x.saturating_sub(1),
            ..self.piece
        });
    }

    pub fn snap_left(&mut self) {
        while self.try_place_piece(Piece {
            x: self.piece.x.saturating_sub(1),
            ..self.piece
        }) {}
    }

    pub fn piece_right(&mut self) {
        self.try_place_piece(Piece {
            x: self.piece.x + 1,
            ..self.piece
        });
    }

    pub fn snap_right(&mut self) {
        while self.try_place_piece(Piece {
            x: self.piece.x + 1,
            ..self.piece
        }) {}
    }

    pub fn rotate_right(&mut self) {
        for (x, y) in get_wallkicks(self.piece.typ, self.piece.rot, self.piece.rot.right()) {
            let newx = self.piece.x as i32 + x;
            let newy = self.piece.y as i32 + y;

            if newx >= 0
                && newy >= 0
                && self.try_place_piece(Piece {
                    x: newx as usize,
                    y: newy as usize,
                    typ: self.piece.typ,
                    rot: self.piece.rot.right(),
                })
            {
                return;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        for (x, y) in get_wallkicks(self.piece.typ, self.piece.rot, self.piece.rot.left()) {
            let newx = self.piece.x as i32 + x;
            let newy = self.piece.y as i32 + y;

            if newx > 0
                && newy > 0
                && self.try_place_piece(Piece {
                    x: newx as usize,
                    y: newy as usize,
                    typ: self.piece.typ,
                    rot: self.piece.rot.left(),
                })
            {
                return;
            }
        }
    }
}
