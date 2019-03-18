mod constants;
mod rest_utils;
use rest_utils::RESTManager;
mod cli_utils;
use cli_utils::CliManager;
use constants::WIN_CONDITIONS;

enum Cmds {
    Game(GameMsg),
    Exit
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Player {
    A,
    B
}

enum GameMsg {
    InvalidMove((Player, (i32, i32))),
    MoveSuccessful(Player),
    Won(Player),
    TurnFinished(Player),
    NewGame,
    Debug,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum GridStatus {
    Piece(Player),
    Empty}

#[derive(Debug)]
pub struct BoardState {
    data_arr: [GridStatus;9],
}

impl BoardState {
    fn build_idx(coord: (i32, i32)) -> usize {
        let (row, col) = coord;
        (row * 3 + col) as usize
    }
    pub fn init() -> BoardState {
        BoardState { data_arr: [GridStatus::Empty; 9] }
    }
    pub fn set_status(&mut self, coord: (i32, i32), status: GridStatus) {
        self.data_arr[BoardState::build_idx(coord)] = status;
    }
    fn reset(&mut self) {
        for piece in self.data_arr.iter_mut() {
            *piece = GridStatus::Empty;
        }
    }
    fn get_status(&self, coord: (i32, i32)) -> GridStatus {
        self.data_arr[BoardState::build_idx(coord)]
    }

    fn check_win(&self,
                 player: &Player) -> bool {
        WIN_CONDITIONS.into_iter().any(
            |win_cond|
                win_cond.into_iter().all(
                    |coord|
                        match self.get_status(*coord) {
                            GridStatus::Empty => false,
                            GridStatus::Piece(player_grid) => {
                                match (player, player_grid) {
                                    (Player::A, Player::A) => true,
                                    (Player::B, Player::B) => true,
                                    _ => false,
                                }
                            }
                        }
                )
        )
    }
}



struct GameState {
    game_io: CliManager,
    first_player: Player,
    board_state: BoardState,
}

impl GameState {
    fn next_player(old_player: Player) -> Player {
        match old_player {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }
    fn init(manager: CliManager) -> GameState {
        GameState {
            game_io: manager,
            first_player: Player::A,
            board_state: BoardState::init(),
        }
    }

    fn new_turn(&mut self, player: Player) -> GameMsg {
        let coord = self.game_io.get_coord(&player);
        match self.board_state.get_status(coord) {
            GridStatus::Empty => {
                // self.history.push(coord);
                let piece = GridStatus::Piece(player.clone());
                self.board_state.set_status(coord, piece);
                GameMsg::MoveSuccessful(player)
            },
            GridStatus::Piece(_) => {
                GameMsg::InvalidMove((player, coord))
            },
        }
    }

    fn update(&mut self, msg: GameMsg) -> Cmds{
        match msg {
            GameMsg::MoveSuccessful(player) => {
                self.game_io.display_board(&self.board_state);
                if self.board_state.check_win(&player) {
                    Cmds::Game(GameMsg::Won(player))
                } else {
                    let new_player = GameState::next_player(player);
                    Cmds::Game(GameMsg::TurnFinished(new_player))
                }
            },
            GameMsg::TurnFinished(player) => {
                Cmds::Game(self.new_turn(player))
            },
            GameMsg::Won(player) => {
                if self.game_io.get_exit_cmd(&player){
                    Cmds::Exit
                } else {
                    Cmds::Game(GameMsg::NewGame)
                }
            }
            GameMsg::InvalidMove((player, coord)) => {
                self.game_io.invalid_move_notification(&player, coord);
                Cmds::Game(GameMsg::TurnFinished(player))
            },
            GameMsg::NewGame => {
                self.board_state.reset();
                self.game_io.game_start_notification();
                self.game_io.display_board(&self.board_state);
                match self.first_player {
                    Player::A => Cmds::Game(self.new_turn(Player::A)),
                    Player::B => Cmds::Game(self.new_turn(Player::B)),

                }
            },
            GameMsg::Debug => {Cmds::Exit}
        }
    }

}

fn main() {
    let cli_manager = CliManager::init();
    let rest_manager = RESTManager::init();
    rest_manager.start();
    let mut game = GameState::init(cli_manager);
    let mut cmd = Cmds::Game(GameMsg::NewGame);
    loop {
        match cmd {
            Cmds::Game(msg) => cmd = game.update(msg),
            Cmds::Exit => break,
        }
    }
}
