from env.custom_hopper import *
from stable_baselines3.common.evaluation import evaluate_policy
from stable_baselines3.common.results_plotter import load_results, ts2xy
import collections
import os

import matplotlib
matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

def moving_average(values, window):
    """
    Smooth values by doing a moving average
    :param values: (numpy array)
    :param window: (int)
    :return: (numpy array)
    """
    weights = np.repeat(1.0, window) / window
    return np.convolve(values, weights, "valid")

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
    plt.show()

def cheetah_full_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]

    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")
    
    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()

        if key in prob:
            v=hopper_dict.get("policy").get(key)
            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j]=v[i,j]
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+3]=v[i,j]
                            dummy_tensor[i,j+6]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        dummy_tensor[i,j]=-v[i,j]
                        dummy_tensor[i+3,j]=v[i,j]
            elif key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=-v[i]
                    dummy_tensor[i+3]=v[i]
            elif key=="log_std":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=v[i]
                    dummy_tensor[i+3]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def cheetah_back_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]

    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")

    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()
        if key in prob:
            v=hopper_dict.get("policy").get(key)
            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+3]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        dummy_tensor[i,j]=v[i,j]
            elif key=="log_std":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=v[i]
            elif key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def cheetah_front_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]

    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")

    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()
        if key in prob:
            v=hopper_dict.get("policy").get(key)
            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+6]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        dummy_tensor[i+3,j]=v[i,j]
            elif key=="log_std" or key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i+3]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def walker_full_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]

    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")

    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()

        if key in prob:
            v=hopper_dict.get("policy").get(key)
            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j]=v[i,j]
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+3]=v[i,j]
                            dummy_tensor[i,j+6]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        dummy_tensor[i,j]=v[i,j]
                        dummy_tensor[i+3,j]=v[i,j]
            elif key=="log_std" or key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=v[i]
                    dummy_tensor[i+3]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def walker_right_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]

    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")

    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()
        if key in prob:
            v=hopper_dict.get("policy").get(key)

            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+3]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==v.size()[1]-1:
                            dummy_tensor[i,j]=v[i,j]
                        else:
                            dummy_tensor[i,j]=v[i,j]
            elif key=="log_std" or key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def walker_left_transfer(hopper_dict,transfer_dict,log=True,value_net=False):
    
    prob=["mlp_extractor.policy_net.0.weight","mlp_extractor.policy_net.0.bias","mlp_extractor.policy_net.2.weight","mlp_extractor.policy_net.2.bias","action_net.weight","action_net.bias"]
    
    if log:
        prob.append("log_std")
    if value_net:
        prob.append("mlp_extractor.value_net.0.weight")
        prob.append("mlp_extractor.value_net.0.bias")
        prob.append("mlp_extractor.value_net.2.weight")
        prob.append("mlp_extractor.value_net.2.bias")
        prob.append("value_net.weight")
        prob.append("value_net.bias")

    dummy_dict=collections.OrderedDict()

    dummy_policy=collections.OrderedDict()
    for key,value in transfer_dict.get("policy").items():
        dummy_tensor=value.clone()

        if key in prob:
            v=hopper_dict.get("policy").get(key)
            if key=="mlp_extractor.policy_net.0.weight" or key=="mlp_extractor.value_net.0.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        if j==0 or j==1:
                            dummy_tensor[i,j]=v[i,j]
                        elif j>1 and j<=4:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j==5 or j==6 or j==7:
                            dummy_tensor[i,j+3]=v[i,j]
                        elif j>7:
                            dummy_tensor[i,j+6]=v[i,j]
            elif key=="action_net.weight":
                for i in range(0,v.size()[0]):
                    for j in range(0,v.size()[1]):
                        dummy_tensor[i+3,j]=v[i,j]
            elif key=="log_std" or key=="action_net.bias":
                for i in range(0,v.size()[0]):
                    dummy_tensor[i+3]=v[i]
            else:
                dim=v.dim()
                if dim==1:
                    for i in range(0,v.size()[0]):
                        dummy_tensor[i]=v[i]
                else:
                    for i in range(0,v.size()[0]):
                        for j in range(0,v.size()[1]):
                            dummy_tensor[i,j]=v[i,j]
            
        dummy_policy[key]=dummy_tensor
    dummy_dict["policy"]=dummy_policy
    dummy_dict["policy.optimizer"]=transfer_dict.get("policy.optimizer")

    return dummy_dict

def train(timesteps,model,path,phrase):
    if os.path.isfile(path+".zip"):
        model.set_parameters(path)
        print(phrase+" downloaded")
    else:
        print("Training of the "+phrase)
        model.learn(total_timesteps=timesteps)
        model.save(path)
        print("Training of the "+phrase+" ended")

def evaluation(model,env,phrase,eval_episodes=50,render=False):
    mean_reward, std_reward = evaluate_policy(model=model,env=env,n_eval_episodes=eval_episodes,render=render)
    print(phrase+": ",mean_reward,std_reward)

'''
log_std torch.Size([6])
mlp_extractor.policy_net.0.weight torch.Size([64, 17])
mlp_extractor.policy_net.0.bias torch.Size([64])
mlp_extractor.policy_net.2.weight torch.Size([64, 64])
mlp_extractor.policy_net.2.bias torch.Size([64])
mlp_extractor.value_net.0.weight torch.Size([64, 17])
mlp_extractor.value_net.0.bias torch.Size([64])
mlp_extractor.value_net.2.weight torch.Size([64, 64])
mlp_extractor.value_net.2.bias torch.Size([64])
action_net.weight torch.Size([6, 64])
action_net.bias torch.Size([6])
value_net.weight torch.Size([1, 64])
value_net.bias torch.Size([1])
'''