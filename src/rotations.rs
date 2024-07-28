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
        RotationState::None => [(1, 0), (1, 1), (1, 2), (0, 1)],
        RotationState::Right => [(0, 1), (1, 1), (2, 1), (2, 2)],
        RotationState::Flip => [(1, 0), (1, 1), (1, 2), (2, 0)],
        RotationState::Left => [(0, 0), (0, 1), (1, 1), (2, 1)],
    }
}

pub fn o_coords(rot: RotationState) -> [(usize, usize); 4] {
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

pub fn get_coords(typ: PieceType, rot:RotationState) -> [(usize, usize); 4] {
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
