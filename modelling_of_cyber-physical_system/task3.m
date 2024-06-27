%% Task 3: Target localization with real data - No Attack
clc
close all
clear all
format default

% CONSTANTS AND PARAMETERS
run('data/task3data.m');  % y: runtime sensor measurements, D: training dictionary
q = 6;  % sensors
p = 7;  % cells
delta = 1e-12; % stop criterion threshold
D_n = normalize(D); % normalized training dictionary 
epsilon = 1e-8; % error on learning rate
tau = norm(D)^(-2)-epsilon; % learning rate
lambda = 1; % sparsity coefficient
Lambda = lambda*ones(p,1); % sparsity array

% ITERATIVE SHRINKAGE THRESHOLDING ALGORITHM
x_est = zeros(1,p)'; % x_est = x_t signal magnitude for p cells
Tmax = 1e8; % max number of IST algorithm iterations
t = 1; % iteration counter
while t < Tmax % general prune condition        
    x_old = x_est; % x_old = x(t)
    parameter = x_old + tau*D_n'*(y-D_n*x_old); % parameter to be passed to the IST
    x_est = f_shrinkage(parameter,Lambda,tau); % x_est = x(t+1)
    t = t + 1; 
    % particular prune condition
    if norm(x_est-x_old) < delta     
        Tmax = t;
        break;
    end
end

% CLEANING
cleaning_threshold = 0.5;
% estimated state trim down
x_est(abs(x_est)<=cleaning_threshold) = 0;

% TASK 3.1 RESULTS
fprintf('Task 3: Target localization with real data - No Attack\n')
fprintf('\na) In the case no sensor got attacked: \n')
fprintf('Estimated State:')
x_est % estimated state
[target, cell] = max(x_est); % [max{state value}=1, position of the target]
fprintf("It has been detected 1 target in %d-th cell\n\n\n\n", cell);


%% Task 3: Target localization with real data - Under Attack
% CONSTANTS AND PARAMETERS
lambda = 1; % sparsity coefficient
I = eye(q); % attack output matrix
G = normalize([D I]); % output matrix of the attacked measurements (partial lasso)
Lambda = lambda*ones(q+p,1); % sparsity array
success = 0; % number of correct predictions

% TARGET LOCALIZATION WITH AN ATTACK ON EACH SENSOR PER TIME 
fprintf('\nTask 3: Target localization with real data - Under Attack \n')
for s = 1:q % s as attacked sensor within the q measures
    h = 1;  % single sensor attack
    a = zeros(q,1); % real attack
    a(s) = y(s)/5; % proportional attack magnitude
    y_a = y + a; % attacked output

    % ITERATIVE SHRINKAGE THRESHOLDING ALGORITHM
    x_a_est = zeros(q+p,1); % estimated state and attacks
    t = 1; % iteration counter
    Tmax = 1e8; % max number of IST algorithm iterations
    while t < Tmax % general prune condition     
        x_a_old = x_a_est; % x_a_old = x_a(t)
        parameter = x_a_old + tau*G'*(y_a-G*x_a_old); % to be passed to the IST
        x_a_est = f_shrinkage(parameter,Lambda,tau); % x_a_est = x_a(t+1)
        t = t + 1;
        if norm(x_a_est-x_a_old) < delta % particular prune condition
            Tmax = t;
            break;
        end
    end
    
    x_est = x_a_est(1:p); % estimated state estraction
    a_est = x_a_est(p+1:q+p); % estimated attack estraction

    % CLEANING
    cleaning_threshold = 0.5; % 
    % estimated state trim down
    x_est(abs(x_est)<=cleaning_threshold) = 0;
    % estimated attack trim down
    a_est(abs(a_est)<=cleaning_threshold) = 0;

    % TASK 3.2 RESULTS
    fprintf('\nb.%d) Single sensor attack which perturbs y(%d) of a quantity equal to y(%d)/5:', s, s, s)
    fprintf('\nEstimated State:')
    x_est % estimate of the state
    [target, cell] = max(x_est); % [max{state value}=1, position of the target]
    fprintf('\nEstimated Attack:')
    a_est % estimate of the attack
    [a_est_magnitude, a_est_sensors] = maxk(abs(a_est), h); % [maxk{attacks}, attacked sensors]
    % estimated attacked sensors == number of real attacked sensors => success
    if a_est_sensors == s  
        attack_detection = "attack correctly detected"
        success = success + 1;
    else
        attack_detection = "attack not detected"
    end
    fprintf("It has been detected a target in %d-th cell while it has been predicted %d-th sensor got attacked, considering the real attacked sensor is the %d-th \n\n", cell, a_est_sensors, s);
    
end
