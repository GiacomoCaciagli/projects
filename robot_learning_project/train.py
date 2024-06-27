"""Sample script for training a control policy on the Hopper environment

    Read the stable-baselines3 documentation and implement a training
    pipeline with an RL algorithm of your choice between TRPO, PPO, and SAC.
"""
import gym
from env.custom_hopper import *
from stable_baselines3 import PPO, SAC
from stable_baselines3.common.evaluation import evaluate_policy
from stable_baselines3.common.monitor import Monitor
from stable_baselines3.common.results_plotter import load_results, ts2xy
import os

import matplotlib
matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

def plot_results(log_folder, title="Learning Curve"):
    """
    plot the results

    :param log_folder: (str) the save location of the results to plot
    :param title: (str) the title of the task to plot
    """
    x, y = ts2xy(load_results(log_folder), "timesteps")
    y = moving_average(y, window=50)
    # Truncate x
    x = x[len(x) - len(y) :]

    fig = plt.figure(title)
    plt.plot(x, y)
    plt.xlabel("Number of Timesteps")
    plt.ylabel("Rewards")
    plt.title(title + " Smoothed")
    plt.show(block=False)

def moving_average(values, window):
    """
    Smooth values by doing a moving average
    :param values: (numpy array)
    :param window: (int)
    :return: (numpy array)
    """
    weights = np.repeat(1.0, window) / window
    return np.convolve(values, weights, "valid")


def main():
    source_env=gym.make('CustomHopper-source-v0')
    target_env=gym.make('CustomHopper-target-v0')

    print('State space:', source_env.observation_space)  # state-space
    print('Action space:', source_env.action_space)  # action-space
    print('Dynamics source parameters:', source_env.get_parameters())  # masses of each link of the Hopper
    print('Dynamics target parameters:', target_env.get_parameters())  # masses of each link of the Hopper

    source_log_dir = "./tmp/gym/source/"
    target_log_dir ="./tmp/gym/target/"

    source_env = Monitor(source_env, source_log_dir)
    target_env = Monitor(target_env, target_log_dir)


    """
        TODO:

            - train a policy with stable-baselines3 on the source Hopper env
            - test the policy with stable-baselines3 on <source,target> Hopper envs (hint: see the evaluate_policy method of stable-baselines3)
    """

    source_model=PPO("MlpPolicy",source_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)
    target_model=PPO("MlpPolicy",target_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)

    if os.path.isfile("ppo_target_model.zip"):
        target_model.set_parameters("ppo_target_model")
        print("Target model downloaded")
    else:
        print("Training of the target model")
        target_model.learn(total_timesteps=250000)
        target_model.save("ppo_target_model")
        print("Training of the target model ended")
    if os.path.isfile("ppo_source_model.zip"):
        source_model.set_parameters("ppo_source_model")
        print("Source model downloaded")
        train=False
    else:
        print("Training of the source model")
        source_model.learn(total_timesteps=250000)
        source_model.save("ppo_source_model")
        train=True
        print("Training of the source model ended")
    
    first_mean_reward, first_std_reward = evaluate_policy(model=source_model,env=source_env,n_eval_episodes=50)
    second_mean_reward, second_std_reward = evaluate_policy(model=source_model,env=target_env,n_eval_episodes=50)
    third_mean_reward, third_std_reward = evaluate_policy(model=target_model,env=target_env,n_eval_episodes=50)

    print('First:',first_mean_reward,first_std_reward)
    print('Second:',second_mean_reward,second_std_reward)
    print('Third:',third_mean_reward,third_std_reward)

    if train:
        plot_results(source_log_dir)
        plt.show()


if __name__ == '__main__':
    main()

'''First: 1604.6820970600002 14.540890991897369
Second: 899.84912822 17.330576825022035
Third: 1056.05404862 132.7571457843102'''