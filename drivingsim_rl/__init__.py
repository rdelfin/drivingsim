import gymnasium as gym
from dataclasses import dataclass
import drivingsim
import numpy as np
import math
from PIL import Image
import io
from typing import Optional

@dataclass
class RewardPoint:
    position: tuple[float, float]
    reward: float


class RandomDrivingSimEnv(gym.Env):
    def __init__(self) -> None:
        self.frames_taken = 0.0
        self.duration_s = 30.0
        self.fps = 25.0
        self.action_space = gym.spaces.Box(low=np.array([-np.inf, -math.pi]), high=np.array([np.inf, math.pi]), dtype=np.float32)
        self.sim = drivingsim.Simulator(drivingsim.VehicleState(), [])
        self.observation_space = gym.spaces.Dict(
            {
                "speed": gym.spaces.Box(low=-np.inf, high=np.inf, shape=(1,), dtype=np.float64),
                "image": gym.spaces.Box(low=0, high=255, shape=(1280, 800, 3), dtype=np.uint8),
            }
        )


    def reset(self, seed: Optional[int] = None, options: Optional[dict] = None):
        super().reset(seed=seed)

        self.frames_taken = 0.0
        vehicle_state = drivingsim.VehicleState()
        self.rewards = [(self.np_random.uniform(50, 1230), self.np_random.uniform(50, 750), 10.0) for _ in range(5)]
        self.sim = drivingsim.Simulator(vehicle_state, self.rewards)

        obs = self._get_observation()
        info = self._get_info()

        return obs, info

    def step(self, action):
        action = drivingsim.Action(action[0], action[1])
        reward = self.sim.advance_s(action, 1.0 / self.fps)
        obs = self._get_observation()
        info = self._get_info()
        self.frames_taken += 1
        time_passed_s = self.frames_taken / self.fps
        terminated = time_passed_s >= self.duration_s
        truncated = False

        return obs, reward, terminated, truncated, info


    def _get_observation(self):
        state = self.sim.get_state_clone()
        vehicle_state = state.get_vehicle_state_clone()
        image_data = state.gen_image_png()
        image = Image.open(io.BytesIO(image_data))
        image_arr = np.asarray(image)
        return {
            "speed": np.array([vehicle_state.speed]),
            "image": image_arr,
        }


    def _get_info(self):
        return dict()


def register_driving_sim():
    gym.register(
        id="gymnasium_env/RandomDrivingSim-v0",
        entry_point=RandomDrivingSimEnv,
    )
