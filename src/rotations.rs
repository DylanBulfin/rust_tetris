use crate::field::{PieceType, RotationState};

pub fn i_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [[false; 4], [true; 4], [false; 4], [false; 4]],
        RotationState::Right => [
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
        ],
        RotationState::Flip => [[false; 4], [false; 4], [true; 4], [false; 4]],
        RotationState::Left => [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    }
}

pub fn j_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [true, false, false, false],
            [true, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, true, true, false],
            [false, true, false, false],
            [false, true, false, false],
            [false; 4],
        ],
        RotationState::Flip => [
            [false; 4],
            [true, true, true, false],
            [false, false, true, false],
            [false; 4],
        ],
        RotationState::Left => [
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false; 4],
        ],
    }
}

pub fn l_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [false, false, true, false],
            [true, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false; 4],
        ],
        RotationState::Flip => [
            [false; 4],
            [true, true, true, false],
            [true, false, false, false],
            [false; 4],
        ],
        RotationState::Left => [
            [true, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false; 4],
        ],
    }
}

pub fn o_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [false, true, true, false],
            [false, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, true, true, false],
            [false, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Flip => [
            [false, true, true, false],
            [false, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Left => [
            [false, true, true, false],
            [false, true, true, false],
            [false; 4],
            [false; 4],
        ],
    }
}

pub fn s_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [false, true, true, false],
            [true, true, false, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false; 4],
        ],
        RotationState::Flip => [
            [false; 4],
            [false, true, true, false],
            [true, true, false, false],
            [false; 4],
        ],
        RotationState::Left => [
            [true, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false; 4],
        ],
    }
}

pub fn z_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [true, true, false, false],
            [false, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, false, true, false],
            [false, true, true, false],
            [false, true, false, false],
            [false; 4],
        ],
        RotationState::Flip => [
            [false; 4],
            [true, true, false, false],
            [false, true, true, false],
            [false; 4],
        ],
        RotationState::Left => [
            [false, true, false, false],
            [true, true, false, false],
            [true, false, false, false],
            [false; 4],
        ],
    }
}

pub fn t_matrix(rot: RotationState) -> [[bool; 4]; 4] {
    match rot {
        RotationState::None => [
            [false, true, false, false],
            [true, true, true, false],
            [false; 4],
            [false; 4],
        ],
        RotationState::Right => [
            [false, true, false, false],
            [false, true, true, false],
            [false, true, false, false],
            [false; 4],
        ],
        RotationState::Flip => [
            [false; 4],
            [true, true, true, false],
            [false, true, false, false],
            [false; 4],
        ],
        RotationState::Left => [
            [false, true, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false; 4],
        ],
    }
}

pub fn get_rotations(typ: PieceType, rot: RotationState) -> [[bool; 4]; 4] {
    match typ {
        PieceType::I => i_matrix(rot),
        PieceType::J => j_matrix(rot),
        PieceType::L => l_matrix(rot),
        PieceType::O => o_matrix(rot),
        PieceType::Z => z_matrix(rot),
        PieceType::S => s_matrix(rot),
        PieceType::T => t_matrix(rot),
    }
}
