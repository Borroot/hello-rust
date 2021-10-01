use std::io::{self, Write};

fn main() -> () {
    let mut onturn: usize = 0; // X if 0 else O
    let mut board: [u16; 2] = [0b000_000_000, 0b000_000_000];

    display(&board);
    loop {
        let newmove: u8 = input(&board, &onturn);
        update(&mut board, &onturn, newmove);
        display(&board);
        if won(&board, &onturn) {
            break;
        }
        onturn = onturn ^ 1;
    }
}

fn onturn_symbol(onturn: &usize) -> char {
    match onturn {
        0 => 'X',
        _ => 'O',
    }
}

fn isempty(board: &[u16; 2], newmove: u8) -> bool {
    match ((board[0] | board[1]) >> newmove) & 1 {
        1 => false, // non-empty cell
        _ => true,  // empty cell
    }
}

fn input(board: &[u16; 2], onturn: &usize) -> u8 {
    let mut buffer: String = String::new();
    loop {
        print!(
            "Player {}, please make a move (1-9): ",
            onturn_symbol(&onturn)
        );
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        if let Ok(number) = buffer.trim().parse::<u8>() {
            if number >= 1 && number <= 9 && isempty(&board, number - 1) {
                return number - 1;
            }
        }
        buffer.clear();
        println!("Make sure to enter a valid move.");
    }
}

fn update(board: &mut [u16; 2], onturn: &usize, newmove: u8) -> () {
    board[*onturn] |= 1 << newmove;
}

fn won(board: &[u16; 2], onturn: &usize) -> bool {
    if (board[0] | board[1]) ^ 0b111_111_111 == 0 {
        println!("It's a draw!");
        return true;
    }
    let three = |check: &u16| -> bool { board[*onturn] & *check == *check };
    let lines: [u16; 8] = [
        0b000_000_111,
        0b000_111_000,
        0b111_000_000,
        0b001_001_001,
        0b010_010_010,
        0b100_100_100,
        0b100_010_001,
        0b001_010_100,
    ];
    if lines.iter().any(three) {
        println!("Player {} won!", onturn_symbol(onturn));
        return true;
    }
    return false;
}

fn display(board: &[u16; 2]) -> () {
    for i in 0..9 {
        if (board[0] >> i) & 1 == 1 {
            print!("X");
        } else if (board[1] >> i) & 1 == 1 {
            print!("O");
        } else {
            print!(".");
        }
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
}
