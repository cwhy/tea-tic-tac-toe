extern crate mime;
use std::char;

use serde::{Serialize, Deserialize};
use gotham::state::{FromState, State};

use hyper::{Body, Response, StatusCode};

use gotham::helpers::http::response::create_response;
use gotham::router::{builder::*, Router};

pub struct RESTManager {
    addr: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameInfo {
    availableActions: Vec<char>,
    nPlayers: i32,
}

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "hello")
}

pub fn get_game_info(state: State) -> (State, Response<Body>) {
    let res = {
        let game_info = GameInfo {
            availableActions: (1..10)
                .map(|x| char::from_digit(x, 10).unwrap()).collect(),
            nPlayers: 2};
        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&game_info).unwrap(),
        )
    };
    (state, res)
}

pub fn game_new(state: State) -> (State, Response<Body>) {
    let res = {
        let game_info = GameInfo {
            availableActions: (1..10)
                .map(|x| char::from_digit(x, 10).unwrap()).collect(),
            nPlayers: 2};
        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&game_info).unwrap(),
        )
    };
    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        // For the path "/" invoke the handler "say_hello"
        route.get("/").to(say_hello);
        route.get("/game/info").to(get_game_info);
    })
}

impl RESTManager {
    pub fn init() -> RESTManager {
        let addr = "127.0.0.1:7878";
        RESTManager {
            addr,
        }
    }
    pub fn start(&self) {
        println!("Listening for requests at http://{}", self.addr);
        gotham::start(self.addr, router());
    }
}

