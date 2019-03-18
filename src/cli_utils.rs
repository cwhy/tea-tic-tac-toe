use std::io;
use crate::{BoardState, Player};
use crate::GridStatus;

macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

pub fn parse_coord() -> (i32, i32){
    parse_line!(i32, i32)
}

pub fn parse_char() -> char{
    let mut cli_cmd = String::new();
    io::stdin().read_line(&mut cli_cmd)
        .expect("Failed to read line");
    let char_o: char = cli_cmd.trim().parse()
        .expect("Please only type one character");
    char_o
}

pub struct CliManager {
    player_a_char: char,
    player_b_char: char,
}

impl CliManager {
    fn player_char(&self, player: &Player) -> char {
        match player {
            Player::A => self.player_a_char,
            Player::B => self.player_b_char,
        }
    }
    pub fn init() -> CliManager {
        println!("Enter piece symbol for player A (e.g: \'x\'):");
        let char_a = parse_char();
        println!("Player A's piece symbol has been set to: {}", char_a);

        println!("Enter piece symbol for player B (e.g: \'o\'):");
        let mut char_b = parse_char();
        while char_a == char_b {
            println!("Please enter a different piece symbol from player A's");
            char_b = parse_char();
        }
        println!("Player B's piece symbol has been set to: {}", char_b);
        CliManager{
            player_a_char: char_a,
            player_b_char: char_b,
        }
    }
    pub fn get_coord(&self, player: &Player) -> (i32, i32){
        println!("Player {:?} 's turn, enter coordinate separated by space",
                 self.player_char(player));
        parse_coord()
    }
    pub fn get_exit_cmd(&self, player: &Player) -> bool {
        println!("Player {:?} won!", self.player_char(player));
        println!("Enter anything to continue, \"exit\" to exit");
        let mut cli_cmd = String::new();
        io::stdin().read_line(&mut cli_cmd)
            .expect("Failed to read line");
        cli_cmd.trim() == "exit"
    }
    pub fn display_board(&self, board_state: &BoardState) {
        println!("-----------");
        for rows in 0..3 {
            let mut row_str = String::with_capacity(20);
            row_str.push_str("  |");
            for cols in 0..3 {
                let grid_status = board_state.get_status((rows, cols));
                let grid_char = match grid_status {
                    GridStatus::Empty => ' ',
                    GridStatus::Piece(player) => self.player_char(&player),
                };
                row_str.push(grid_char);
                row_str.push('|');
            }
            println!("{}", row_str);
        }
        println!("-----------");
    }
    pub fn invalid_move_notification(&self, _player: &Player, coord: (i32, i32)) {
        println!("Invalid move {:?}, try again", coord);
    }
    pub fn game_start_notification(&self) {
        println!("Game start!");
    }
}


