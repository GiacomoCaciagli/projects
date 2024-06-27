"""Sample script for training a control policy on the Hopper environment

    Read the stable-baselines3 documentation and implement a training
    pipeline with an RL algorithm of your choice between TRPO, PPO, and SAC.
"""
import gym
from env.custom_hopper import *
from stable_baselines3 import PPO
from stable_baselines3.common.monitor import Monitor
from extension_utils import *

def main():

    walker_env=gym.make("Walker2d-v2")
    hopper_env=gym.make("CustomHopper-source-v0")

    print('State space:', walker_env.observation_space)  # state-space
    print('Action space:', walker_env.action_space)  # action-space
    print('Dynamics source parameters:', walker_env.sim.model.body_names)  # masses of each link of the Hopper

    print('Dynamics source parameters:', walker_env.sim.model.body_mass)  # masses of each link of the Hopper

    walker_log_dir = "./tmp/gym/walker/normal/"
    hopper_log_dir="./tmp/gym/walker/hopper/"

    foot=hopper_env.sim.data.get_body_xpos('foot')

    print("foot:",foot)

    walker_env=Monitor(walker_env, walker_log_dir)
    hopper_env=Monitor(hopper_env, hopper_log_dir)

    walker_model=PPO("MlpPolicy",walker_env,verbose=0,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)

    hopper_model=PPO("MlpPolicy",hopper_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)
    udr_hopper_model=PPO("MlpPolicy",hopper_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)

    right_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)
    full_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)
    left_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)
    udr_right_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)
    udr_full_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)
    udr_left_transfer_model=PPO("MlpPolicy",walker_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0002,batch_size=32)

    hopper_model.set_parameters("ppo_source_model")
    udr_hopper_model.set_parameters("udr_ppo_source_model")

    hopper_dict=hopper_model.get_parameters()
    udr_hopper_dict=udr_hopper_model.get_parameters()
    walker_dict=walker_model.get_parameters()

    log_std=True
    value_net=True
    timesteps=int(500000)
    
    right_transfer_model.set_parameters(walker_right_transfer(hopper_dict,walker_dict,log_std,value_net))
    full_transfer_model.set_parameters(walker_full_transfer(hopper_dict,walker_dict,log_std,value_net))
    left_transfer_model.set_parameters(walker_left_transfer(hopper_dict,walker_dict,log_std,value_net))
    udr_right_transfer_model.set_parameters(walker_right_transfer(udr_hopper_dict,walker_dict,log_std,value_net))
    udr_full_transfer_model.set_parameters(walker_full_transfer(udr_hopper_dict,walker_dict,log_std,value_net))
    udr_left_transfer_model.set_parameters(walker_left_transfer(udr_hopper_dict,walker_dict,log_std,value_net))
    
    p="extension_models/walker/"

    if log_std:
        if value_net:
            p=p+"full_transfer/"
        else:
            p=p+"policy_std/"
    else:
        if value_net:
            p=p+"policy_value/"
        else:
            p=p+"policy/"

    if timesteps!=0:
        path="extension_models/walker/base_walker_"+str(timesteps)
        train(timesteps,walker_model,path,"walker base model")
        path=p+"right_transfer_"+str(timesteps)
        train(timesteps,right_transfer_model,path,"right transfer model")
        path=p+"full_transfer_"+str(timesteps)
        train(timesteps,full_transfer_model,path,"full transfer model")
        path=p+"left_transfer_"+str(timesteps)
        train(timesteps,left_transfer_model,path,"left transfer model")
        path=p+"udr_right_transfer_"+str(timesteps)
        train(timesteps,udr_right_transfer_model,path,"udr right transfer model")
        path=p+"udr_full_transfer_"+str(timesteps)
        train(timesteps,udr_full_transfer_model,path,"udr full transfer model")
        path=p+"udr_left_transfer_"+str(timesteps)
        train(timesteps,udr_left_transfer_model,path,"udr left transfer model")
    
    print("Evaluations:")
    evaluation(walker_model,walker_env,"Walker model")
    evaluation(right_transfer_model,walker_env,"Right transfer model")
    evaluation(full_transfer_model,walker_env,"Full transfer model")
    evaluation(left_transfer_model,walker_env,"Left transfer model")
    evaluation(udr_right_transfer_model,walker_env,"Udr right transfer model")
    evaluation(udr_full_transfer_model,walker_env,"Udr full transfer model")
    evaluation(udr_left_transfer_model,walker_env,"Udr left transfer model")
    
if __name__ == '__main__':
    main()



