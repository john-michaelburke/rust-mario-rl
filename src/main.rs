use pyo3::prelude::*;
use pyo3::types::PyModule;

fn main() {
    Python::with_gil(|py| {
        let custom_manager = PyModule::from_code(py, r#"
from nes_py.wrappers import JoypadSpace
import gym_super_mario_bros
from gym_super_mario_bros.actions import SIMPLE_MOVEMENT

class Game:
    def __init__(self):
        self.env = gym_super_mario_bros.make('SuperMarioBros-v0')
        self.env = JoypadSpace(self.env, SIMPLE_MOVEMENT)

    def play(self):
        done = False
        state = self.env.reset()
        while not done:
            state, reward, done, info = self.env.step(self.env.action_space.sample())
            self.env.render()
        self.env.close()

        "#, "game.py", "game").unwrap();

        let game = custom_manager.getattr("Game").unwrap().call1(()).unwrap();

        game.call_method0("play").unwrap();
    })
}
