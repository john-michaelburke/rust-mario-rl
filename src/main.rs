use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::{Python, PyResult, FromPyObject};
use pyo3::types::{PyModule, PyTuple, PyDict};
use serde::{Deserialize, Serialize};


struct Game {

}

#[derive(Debug, FromPyObject)]
pub struct Step {
    state: Vec<Vec<Vec<u8>>>,
    reward: i16,
    done: bool,
    flag_get: bool
}

fn main() {
    Python::with_gil(|py| {
        let custom_manager = PyModule::from_code(py, r#"
from nes_py.wrappers import JoypadSpace
import gym_super_mario_bros
from gym_super_mario_bros.actions import SIMPLE_MOVEMENT
import json

class Game:
    def __init__(self):
        self.env = gym_super_mario_bros.make('SuperMarioBros-v0')
        self.env = JoypadSpace(self.env, SIMPLE_MOVEMENT)
        self.render = True

    def reset(self):
        self.env.reset()

    def rand_action(self):
        return self.env.action_space.sample()

    def step(self, action: int):
        state, reward, done, info = self.env.step(action)
        if self.render:
            self.env.render()
        return (state, reward, bool(done), info['flag_get'])
            
    def close(self):
        self.env.close()

        "#, "game.py", "game").unwrap();

        let game = custom_manager.getattr("Game").unwrap().call1(()).unwrap();
        let mut done;
        loop {
            done = false;
            game.call_method0("reset").unwrap();
            while !done {
                
                let result = game.call_method1("step", (1 as u8,)).unwrap();
                let step = if let Ok((state, reward, done, flag_get)) = result.extract() {
                    Step { state, reward, done, flag_get }
                } else {
                    panic!()
                };
            }
        }
    })
}
