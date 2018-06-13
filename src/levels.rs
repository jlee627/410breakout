// This is the level.rs file for the classic game Breakout,
// which is responsible for defining the levels in the game

pub const LEVELS: [[[i32; 10]; 6]; ::NUMBER_OF_LEVELS as usize + 1] = [
    [
        // Level 0 -- Debug
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // Level 1
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ],
    [
        // Level 2
        [3, 1, 3, 1, 3, 1, 3, 1, 3, 1],
        [1, 3, 1, 3, 1, 3, 1, 3, 1, 3],
        [3, 1, 3, 1, 3, 1, 3, 1, 3, 1],
        [1, 3, 1, 3, 1, 3, 1, 3, 1, 3],
        [3, 1, 3, 1, 3, 1, 3, 1, 3, 1],
        [1, 3, 1, 3, 1, 3, 1, 3, 1, 3],
    ],
    [
        // Level 3
        [1, 3, 1, 3, 1, 3, 1, 3, 1, 3],
        [5, 1, 5, 1, 5, 1, 5, 1, 5, 1],
        [1, 3, 1, 3, 1, 3, 1, 3, 1, 3],
        [4, 1, 4, 1, 4, 1, 4, 1, 4, 1],
        [1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
        [3, 1, 3, 1, 3, 1, 3, 1, 3, 1],
    ],
    [
        // Level 4
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
    ],
    [
        // Level 5
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
    ],
];
