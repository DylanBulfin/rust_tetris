use crate::state::{PieceType, RotationState};

//Everything in this file was taken from https://tetris.wiki/Super_Rotation_System

pub fn i_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(1, 0), (1, 1), (1, 2), (1, 3)],
        RotationState::Right => [(0, 2), (1, 2), (2, 2), (3, 2)],
        RotationState::Flip => [(2, 0), (2, 1), (2, 2), (2, 3)],
        RotationState::Left => [(0, 1), (1, 1), (2, 1), (3, 1)],
    }
}

pub fn j_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(0, 0), (1, 0), (1, 1), (1, 2)],
        RotationState::Right => [(0, 1), (0, 2), (1, 1), (2, 1)],
        RotationState::Flip => [(1, 0), (1, 1), (1, 2), (2, 2)],
        RotationState::Left => [(2, 0), (0, 1), (1, 1), (2, 1)],
    }
}

pub fn l_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(1, 0), (1, 1), (1, 2), (0, 2)],
        RotationState::Right => [(0, 1), (1, 1), (2, 1), (2, 2)],
        RotationState::Flip => [(1, 0), (1, 1), (1, 2), (2, 0)],
        RotationState::Left => [(0, 0), (0, 1), (1, 1), (2, 1)],
    }
}

pub fn o_coords(_: RotationState) -> [(usize, usize); 4] {
    [(0, 1), (1, 1), (0, 2), (1, 2)]
}

pub fn s_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(1, 0), (1, 1), (0, 1), (0, 2)],
        RotationState::Right => [(0, 1), (1, 1), (1, 2), (2, 2)],
        RotationState::Flip => [(1, 1), (1, 2), (2, 0), (2, 1)],
        RotationState::Left => [(0, 0), (1, 0), (1, 1), (2, 1)],
    }
}

pub fn z_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(0, 0), (0, 1), (1, 1), (1, 2)],
        RotationState::Right => [(1, 1), (0, 2), (1, 2), (2, 1)],
        RotationState::Flip => [(1, 0), (1, 1), (2, 1), (2, 2)],
        RotationState::Left => [(0, 1), (1, 0), (1, 1), (2, 0)],
    }
}

pub fn t_coords(rot: RotationState) -> [(usize, usize); 4] {
    match rot {
        RotationState::None => [(0, 1), (1, 0), (1, 1), (1, 2)],
        RotationState::Right => [(0, 1), (1, 1), (1, 2), (2, 1)],
        RotationState::Flip => [(1, 0), (1, 1), (1, 2), (2, 1)],
        RotationState::Left => [(0, 1), (1, 0), (1, 1), (2, 1)],
    }
}

pub fn get_wallkicks(
    typ: PieceType,
    rot: RotationState,
    new_rot: RotationState,
) -> [(i32, i32); 5] {
    match typ {
        PieceType::O => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        PieceType::I => match (rot, new_rot) {
            (RotationState::None, RotationState::Right) => {
                [(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)]
            }
            (RotationState::Right, RotationState::None) => {
                [(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)]
            }
            (RotationState::Right, RotationState::Flip) => {
                [(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)]
            }
            (RotationState::Flip, RotationState::Right) => {
                [(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)]
            }
            (RotationState::Flip, RotationState::Left) => {
                [(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)]
            }
            (RotationState::Left, RotationState::Flip) => {
                [(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)]
            }
            (RotationState::Left, RotationState::None) => {
                [(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)]
            }
            (RotationState::None, RotationState::Left) => {
                [(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)]
            }
            _ => panic!("Attempting to get wallkick for impossible rotation"),
        },
        _ => match (rot, new_rot) {
            (RotationState::None, RotationState::Right) => {
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            }
            (RotationState::Right, RotationState::None) => {
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            }
            (RotationState::Right, RotationState::Flip) => {
                [(0, 0), (-1, 0), (1, 1), (0, -2), (1, -2)]
            }
            (RotationState::Flip, RotationState::Right) => {
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            }
            (RotationState::Flip, RotationState::Left) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
            (RotationState::Left, RotationState::Flip) => {
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            }
            (RotationState::Left, RotationState::None) => {
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            }
            (RotationState::None, RotationState::Left) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
            _ => panic!("Attempting to get wallkick for impossible rotation"),
        },
    }
}

pub fn get_coords(typ: PieceType, rot: RotationState) -> [(usize, usize); 4] {
    match typ {
        PieceType::I => i_coords(rot),
        PieceType::J => j_coords(rot),
        PieceType::L => l_coords(rot),
        PieceType::O => o_coords(rot),
        PieceType::Z => z_coords(rot),
        PieceType::S => s_coords(rot),
        PieceType::T => t_coords(rot),
    }
}
