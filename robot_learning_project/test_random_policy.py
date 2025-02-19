"""Test a random policy on the Gym Hopper environment

    Play around with this code to get familiar with the
    Hopper environment.

    For example, what happens if you don't reset the environment
    even after the episode is over?
    When exactly is the episode over?
    What is an action here?
"""
import gym
from env.custom_hopper import *


def main():
    render = False

    env = gym.make('CustomHopper-source-v0')
    #env = gym.make('CustomHopper-target-v0')

    print('source hopper:')
    print('State space:', env.observation_space)  # state-space
    print('Action space:', env.action_space)  # action-space
    print('Dynamics parameters:', env.get_parameters())  # masses of each link of the Hopper
    print('Body parts:',env.sim.model.body_names)
    print('Masses of the body parts:',env.sim.model.body_mass)
    print('Degrees of freedom:',env.sim.model.nv)
    print('Degrees of freedom for each body part:',env.sim.model.body_dofnum)
    print('Number of actuators:',env.sim.model.nu)

    '''
    env = gym.make('CustomHopper-target-v0')

    print('target hopper:')
    print('State space:', env.observation_space)  # state-space
    print('Action space:', env.action_space)  # action-space
    print('Dynamics parameters:', env.get_parameters())  # masses of each link of the Hopper
    print('')
    '''

    n_episodes = 5

    for ep in range(n_episodes):  
        done = False
        state = env.reset()  # Reset environment to initial state

        while not done:  # Until the episode is over
            action = env.action_space.sample()  # Sample random action

            state, reward, done, info = env.step(action)  # Step the simulator to the next timestep

            if render:
                env.render()


if __name__ == '__main__':
    main()