"""Sample script for training a control policy on the Hopper environment

    Read the stable-baselines3 documentation and implement a training
    pipeline with an RL algorithm of your choice between TRPO, PPO, and SAC.
"""
import gym
from env.custom_hopper import *
from stable_baselines3 import PPO
from stable_baselines3.common.evaluation import evaluate_policy
from stable_baselines3.common.monitor import Monitor
import os

def main():

    source_env=gym.make('CustomHopper-source-v0')
    custom_source_env=gym.make('CustomHopper-udr-v0')
    target_env=gym.make('CustomHopper-target-v0')

    print('State space:', custom_source_env.observation_space)  # state-space
    print('Action space:', custom_source_env.action_space)  # action-space
    print('Dynamics source parameters:', custom_source_env.get_parameters())  # masses of each link of the Hopper
    print('Dynamics target parameters:', target_env.get_parameters())  # masses of each link of the Hopper

    target_model=PPO("MlpPolicy",target_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)
    source_model=PPO("MlpPolicy",custom_source_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)
    source_model_1=PPO("MlpPolicy",source_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)


    if os.path.isfile("ppo_target_model.zip"):
        target_model.set_parameters("ppo_target_model")
        print("Target model downloaded")
    else:
        print("Training of the target model")
        target_model.learn(total_timesteps=250000)
        target_model.save("ppo_target_model")
        print("Training of the target model ended")

    if os.path.isfile("udr_ppo_source_model.zip"):
        source_model.set_parameters("udr_ppo_source_model")
        source_model_1.set_parameters("udr_ppo_source_model")
        print("Source model downloaded")
    else:
        print("Training of the source model")
        source_model.learn(total_timesteps=250000)
        source_model.save("udr_ppo_source_model")
        print("Training of the source model ended")


    a,b=evaluate_policy(model=source_model_1,env=source_env,n_eval_episodes=50)
    second_mean_reward, second_std_reward = evaluate_policy(model=source_model,env=target_env,n_eval_episodes=50)
    third_mean_reward, third_std_reward = evaluate_policy(model=target_model,env=target_env,n_eval_episodes=50)

    print("udr on source:",a,b)
    print('Second:',second_mean_reward,second_std_reward)
    print('Third:',third_mean_reward,third_std_reward)


if __name__ == '__main__':
    main()

