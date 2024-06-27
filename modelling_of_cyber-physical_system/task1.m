%% Task 1: Implementation of IST
clear all
close all
clc
format default;

% CONSTANTS AND PARAMETERS
q = 10; % sensors
p = 20; % state dimension
k = 2; % support of x
epsilon = 1e-8; % learning rate error
sigma = 1e-2; % noise variance
delta = 1e-12; % stop criterion threshold

n_iter = 1000; % number of runs
estimation_results = zeros([1 n_iter]); % array of correct estimations ( 0 = uncorrect, 1 = correct) and true number of iterations
success_counter = 0; % number of correct estimations
success_percentage = 0; % correct estimations over total
min_iterations = 0;
max_iterations = 0;
avg_iterations = 0;

% ITERATIONS
for j = 1:n_iter
    % RANDOM VARIABLES GENERATION
    C = randn(q,p); % output matrix
    tau = norm(C)^(-2)-epsilon; % learning rate
    lambda = 1e-2/tau; % sparsity coefficient
    Lambda = lambda * ones(p,1); % sparsity vector

    % real state
    x = zeros(p,1); 
    support = zeros(k,1);

    for i = 1:k
        % state support random generation 
        support(i) = randi([1,p],1);
        while ismember(support(i), support(1:i-1,1))
            support(i) = randi([1,p],1);                             
        end 
        % state element random generation 
        x(support(i)) = 1 + 2*rand(1);
        x(support(i),x(support(i))>2) = x(support(i)) - 4;
    end

    eta = sigma * randn(q,1); % gaussian noise 
    y = C*x + eta; % real output

    % ITERATIVE SHRINKAGE THRESHOLDING ALGORITHM
    x_est = zeros(p,1); % state estimation x_est = x(t)
    Tmax = 1e6; % max iteration
    t = 1; % iteration counter
    while t < Tmax % general stop condition      
        x_old = x_est; % x_old = x(t)
        parameter = x_old + tau*C'*(y-C*x_old); % to be passed to the IST
        x_est = f_shrinkage(parameter,Lambda,tau); % x_est = x(t+1)
        t = t + 1; 
        % particular stop condition
        if norm(x_est-x_old) < delta     
            Tmax = t;
            break;
        end
    end

    % CLEANING
    cleaning_threshold = 10*tau*lambda; % 0.1
    % nullification of elements outside the threshold interval
    x_est(abs(x_est)<=cleaning_threshold) = 0;
    
    % j-th ITERATION RESULT EVALUATION
    support = sort(support); % real state support
    support_est = find(x_est); % estimated state support 
    vettTmax(j) = Tmax; % total iterations vector
    % boolean support evaluation
    if size(support_est) == size(support) 
        if support_est == support
            success_counter = success_counter + 1; % success counter
            estimation_results(j) = 1; % 1: right estimation, 0: wrong one
        end
    end
end


success_counter
success_percentage = success_counter/n_iter*100
min_iterations = min(vettTmax)
max_iterations = max(vettTmax)
avg_iterations = mean(vettTmax)


