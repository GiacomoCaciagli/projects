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

    cheetah_env=gym.make("HalfCheetah-v2")
    hopper_env=gym.make("CustomHopper-source-v0")

    print('State space:', cheetah_env.observation_space)  # state-space
    print('Action space:', cheetah_env.action_space)  # action-space
    print('Dynamics source parameters:', cheetah_env.sim.model.body_names)  # masses of each link of the Hopper

    print('Dynamics source parameters:', cheetah_env.sim.model.body_mass)  # masses of each link of the Hopper

    cheetah_log_dir = "./tmp/gym/cheeta/normal/"
    hopper_log_dir="./tmp/gym/cheeta/hopper/"

    cheetah_env=Monitor(cheetah_env, cheetah_log_dir)
    hopper_env=Monitor(hopper_env, hopper_log_dir)

    cheetah_model=PPO("MlpPolicy",cheetah_env,verbose=0,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)

    hopper_model=PPO("MlpPolicy",hopper_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)
    udr_hopper_model=PPO("MlpPolicy",hopper_env,verbose=0,gamma=0.99,n_steps=2048,n_epochs=10,seed=5,learning_rate=0.0015)

    back_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)
    full_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)
    front_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)
    udr_back_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)
    udr_front_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)
    udr_full_transfer_model=PPO("MlpPolicy",cheetah_env,verbose=1,gamma=0.99,n_steps=1024,n_epochs=10,seed=3,learning_rate=0.0003,batch_size=64)

    hopper_model.set_parameters("ppo_source_model")
    udr_hopper_model.set_parameters("udr_ppo_source_model")

    hopper_dict=hopper_model.get_parameters()
    udr_hopper_dict=udr_hopper_model.get_parameters()
    cheetah_dict=cheetah_model.get_parameters()

    log_std=True
    value_net=True
    timesteps=int(500000)

    back_transfer_model.set_parameters(cheetah_back_transfer(hopper_dict,cheetah_dict,log_std,value_net))
    full_transfer_model.set_parameters(cheetah_full_transfer(hopper_dict,cheetah_dict,log_std,value_net))
    front_transfer_model.set_parameters(cheetah_front_transfer(hopper_dict,cheetah_dict,log_std,value_net))
    udr_back_transfer_model.set_parameters(cheetah_back_transfer(udr_hopper_dict,cheetah_dict,log_std,value_net))
    udr_full_transfer_model.set_parameters(cheetah_full_transfer(udr_hopper_dict,cheetah_dict,log_std,value_net))
    udr_front_transfer_model.set_parameters(cheetah_front_transfer(udr_hopper_dict,cheetah_dict,log_std,value_net))

    p="extension_models/cheetah/"

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
        path="extension_models/cheetah/base_cheetah_"+str(timesteps)
        train(timesteps,cheetah_model,path,"cheetah base model")
        path=p+"back_transfer_"+str(timesteps)
        train(timesteps,back_transfer_model,path,"back transfer model")
        path=p+"full_transfer_"+str(timesteps)
        train(timesteps,full_transfer_model,path,"full transfer model")
        path=p+"front_transfer_"+str(timesteps)
        train(timesteps,front_transfer_model,path,"front transfer model")
        path=p+"udr_back_transfer_"+str(timesteps)
        train(timesteps,udr_back_transfer_model,path,"udr back transfer model")
        path=p+"udr_full_transfer_"+str(timesteps)
        train(timesteps,udr_full_transfer_model,path,"udr full transfer model")
        path=p+"udr_front_transfer_"+str(timesteps)
        train(timesteps,udr_front_transfer_model,path,"udr front transfer model")
        
    print("Evaluations:")
    evaluation(cheetah_model,cheetah_env,"Cheeta model")
    evaluation(back_transfer_model,cheetah_env,"Back transfer model")
    evaluation(full_transfer_model,cheetah_env,"Full transfer model")
    evaluation(front_transfer_model,cheetah_env,"Front transfer model")
    evaluation(udr_back_transfer_model,cheetah_env,"Udr back transfer model")
    evaluation(udr_full_transfer_model,cheetah_env,"Udr full transfer model")
    evaluation(udr_front_transfer_model,cheetah_env,"Udr front transfer model")
    
if __name__ == '__main__':
    main()
