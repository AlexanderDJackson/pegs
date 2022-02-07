use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

fn print_board(board: u16) {
    println!("    {:b}    ", board >> 15);

    print!("   {:b} ", (board << 1) >> 15);
    print!("{:b}   \n", (board << 2) >> 15);

    print!("  {:b} ", (board << 3) >> 15);
    print!("{:b} ", (board << 4) >> 15);
    print!("{:b}\n", (board << 5) >> 15);

    print!(" {:b} ", (board << 6) >> 15);
    print!("{:b} ", (board << 7) >> 15);
    print!("{:b} ", (board << 8) >> 15);
    print!("{:b} \n", (board << 9) >> 15);

    print!("{:b} ", (board << 10) >> 15);
    print!("{:b} ", (board << 11) >> 15);
    print!("{:b} ", (board << 12) >> 15);
    print!("{:b} ", (board << 13) >> 15);
    print!("{:b}\n", (board << 14) >> 15);
}

fn possible_positions(board: u16, transitions: &[[u16; 4]; 18]) -> Vec<u16> {
    let mut positions: Vec<u16> = Vec::new();

    // loop through all possible transitions
    for i in 0..18 {
        // check if the transition is possible
        // check if there is an empty space followed by two occupied spaces
        if (board & transitions[i][1] != 0
            && board & transitions[i][2] != 0
            && !board & transitions[i][3] != 0)
            // or check if there are two occupied spaces followed by an empty space
            || (!board & transitions[i][1] != 0
                && board & transitions[i][2] != 0
                && board & transitions[i][3] != 0)
        {
            positions.push(board ^ transitions[i][0]);
        }
    }

    positions
}

fn generateBranch(
    num_tabs: usize,
    file_name: &String,
    board: u16,
    transitions: &[[u16; 4]; 18],
) -> () {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
    for _ in 0..num_tabs {
        write!(file, "\t").unwrap();
    }

    file.write_all(format!("{:b}\n", board).as_bytes()).unwrap();

    let positions = possible_positions(board, &transitions);

    if positions.len() != 0 {
        for position in positions {
            generateBranch(num_tabs + 1, &file_name, position, &transitions);
        }
    }
}

fn main() {
    let transitions: [[u16; 4]; 18] = [
        // / slant
        [
            // The mask to xor with
            0b1_10_100_0000_00000_0,
            // check for the first bit
            0b1_00_000_0000_00000_0,
            // check for the second bit
            0b0_10_000_0000_00000_0,
            // check for the third bit
            0b0_00_100_0000_00000_0,
        ],
        [
            0b0_10_100_1000_00000_0,
            0b0_10_000_0000_00000_0,
            0b0_00_100_0000_00000_0,
            0b0_00_000_1000_00000_0,
        ],
        [
            0b0_00_100_1000_10000_0,
            0b0_00_100_0000_00000_0,
            0b0_00_000_1000_00000_0,
            0b0_00_000_0000_10000_0,
        ],
        [
            0b0_01_010_0100_00000_0,
            0b0_01_000_0000_00000_0,
            0b0_00_010_0000_00000_0,
            0b0_00_000_0100_00000_0,
        ],
        [
            0b0_00_010_0100_01000_0,
            0b0_00_010_0000_00000_0,
            0b0_00_000_0100_00000_0,
            0b0_00_000_0000_01000_0,
        ],
        [
            0b0_00_001_0010_00100_0,
            0b0_00_001_0000_00000_0,
            0b0_00_000_0010_00000_0,
            0b0_00_000_0000_00100_0,
        ],
        // \ slant
        [
            0b1_01_001_0000_00000_0,
            0b1_00_000_0000_00000_0,
            0b0_01_000_0000_00000_0,
            0b0_00_001_0000_00000_0,
        ],
        [
            0b0_01_001_0001_00000_0,
            0b0_01_000_0000_00000_0,
            0b0_00_001_0000_00000_0,
            0b0_00_000_0001_00000_0,
        ],
        [
            0b0_00_001_0001_00001_0,
            0b0_00_001_0000_00000_0,
            0b0_00_000_0001_00000_0,
            0b0_00_000_0000_00001_0,
        ],
        [
            0b0_10_010_0010_00000_0,
            0b0_10_000_0000_00000_0,
            0b0_00_010_0000_00000_0,
            0b0_00_000_0010_00000_0,
        ],
        [
            0b0_00_010_0010_00010_0,
            0b0_00_010_0000_00000_0,
            0b0_00_000_0010_00000_0,
            0b0_00_000_0000_00010_0,
        ],
        [
            0b0_00_100_0100_00100_0,
            0b0_00_100_0000_00000_0,
            0b0_00_000_0100_00000_0,
            0b0_00_000_0000_00100_0,
        ],
        // horizontal
        [
            0b0_00_111_0000_00000_0,
            0b0_00_100_0000_00000_0,
            0b0_00_010_0000_00000_0,
            0b0_00_001_0000_00000_0,
        ],
        [
            0b0_00_000_1110_00000_0,
            0b0_00_000_1000_00000_0,
            0b0_00_000_0100_00000_0,
            0b0_00_000_0010_00000_0,
        ],
        [
            0b0_00_000_0111_00000_0,
            0b0_00_000_0100_00000_0,
            0b0_00_000_0010_00000_0,
            0b0_00_000_0001_00000_0,
        ],
        [
            0b0_00_000_0000_11100_0,
            0b0_00_000_0000_10000_0,
            0b0_00_000_0000_01000_0,
            0b0_00_000_0000_00100_0,
        ],
        [
            0b0_00_000_0000_01110_0,
            0b0_00_000_0000_01000_0,
            0b0_00_000_0000_00100_0,
            0b0_00_000_0000_00010_0,
        ],
        [
            0b0_00_000_0000_00111_0,
            0b0_00_000_0000_00100_0,
            0b0_00_000_0000_00010_0,
            0b0_00_000_0000_00001_0,
        ],
    ];

    let starting_positions: [u16; 15] = [
        0b0_11_111_1111_11111_0,
        0b1_01_111_1111_11111_0,
        0b1_10_111_1111_11111_0,
        0b1_11_011_1111_11111_0,
        0b1_11_101_1111_11111_0,
        0b1_11_110_1111_11111_0,
        0b1_11_111_0111_11111_0,
        0b1_11_111_1011_11111_0,
        0b1_11_111_1101_11111_0,
        0b1_11_111_1110_11111_0,
        0b1_11_111_1111_01111_0,
        0b1_11_111_1111_10111_0,
        0b1_11_111_1111_11011_0,
        0b1_11_111_1111_11101_0,
        0b1_11_111_1111_11110_0,
    ];

    /*
    let mut board = Board::new();
    for i in 0..15 {
        let mut file = OpenOptions::new()
            .append(true)
            .open(format!("solutions/{}.txt", i))
            .expect("cannot open file");
        file.write_all("Hello World".as_bytes())
            .expect("write failed");
    }

    print_board(0b1_00_110_1001_10111_0);
    for i in possible_positions(0b1_00_110_1001_10111_0, &transitions) {
        print_board(i);
    }

    for i in possible_positions(0b1001100011100100, &transitions) {
        println!("{:0>16b}", i);
    }

    generateBranch(
        0,
        &String::from("solutions/0.txt"),
        0b0_11_111_1111_11111_0,
        &transitions,
    );
    */

    //This is running for a long time
    //I might have underestimated the number of possible positions
}
