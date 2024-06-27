%% Task 2: Secure estimation under sparse sensor attack
clc
clear all
close all
format default

% CONSTANTS AND PARAMETERS
epsilon = 1e-8; % error on tau 
n = 10; % state dimension
q = 20; % number of sensors
h = 2; % sensor attacks
sigma = 1e-2; % Gaussian standard deviation of the noise
delta = 1e-12; % stop criterion threshold

for instance = 1:4
    fprintf('\n%d)', instance)
    switch instance
        case 1
            fprintf(' No noise and Unaware attack:\n')
            eta = 0; % noise
        case 2
            fprintf(' Gaussian noise and Unaware attack:\n')
            eta = sigma*randn(1,q)'; % noise
        case 3
            fprintf(' No noise and Aware attack:\n')
            eta = 0; % noise
        case 4
            fprintf(' Gaussian noise and Aware attack:\n')
            eta = sigma*randn(1,q)'; % noise
    end

    n_iter = 1000; % number of runs
    a_estimation_results = zeros(n_iter,1); % array of correct estimations (0 = uncorrect, 1 = correct)
    x_estimation_accuracy = zeros(n_iter,1); % array of mean squared errors
    a_success_counter = 0; % number of correct estimations
    a_success_percentage = 0; % correct estimations over total
    
    % ITERATIONS
    for j = 1:n_iter
        % RANDOM VARIABLES GENERATION
        C = randn(q,n); % output matrix
        x = randn(n,1); % real state
        a = zeros(q,1); % real attack
        a_support = zeros(h,1); % support of the attacks
        y = C*x + eta; % real measurements
        
        % ATTACK GENERATION
        % support random generation of the attack
        for i = 1:h
            % attack support element generation
            a_support(i) = randi([1,q],1);
            while ismember(a_support(i), a_support(1:i-1))
                a_support(i) = randi([1,q],1);                     
            end
            % attack generation
            if instance <= 2
                % unaware attack
                while abs(a(a_support(i))) < 1 
                    a(a_support(i)) = -2 + (2*2)*rand(1); % element random generation in [-2,-1]U[1,2] interval
                end
            else
                % aware attack
                a(a_support(i)) = y(a_support(i))/2; % depending attack on sensor
            end  
        end

        y = y + a; % attacked measurements
        I = eye(q); % output attack matrix
        G = [C I]; % output matrix of the attacked measurements (matrix of the partial lasso)
        tau = norm(G)^(-2)-epsilon; % learning rate
        lambda = 1e-3/tau; % sparsity coefficient
        Lambda = [zeros(n,1);ones(q,1)]; 
        Lambda = lambda * Lambda; % sparsity array
        x_a = [x; a]; % real array of state and attacks [state; attack]
    
        % ITERATIVE SHRINKAGE THRESHOLDING ALGORITHM
        x_est = zeros(n,1); % estimated state
        a_est = zeros(q,1); % estimated attacks
        x_a_est = [x_est; a_est]; % estimated array of state and attacks [estimated state; estimated attacks]
        Tmax = 1e6; % maximum number of IST algorithm iterations
        t = 1; % iteration counter
        while t < Tmax % general prune condition           
            x_a_old = x_a_est; % x_a_old = x_a(t) 
            parameter = x_a_old + tau*G'*(y-G*x_a_old); % to be passed to the IST
            x_a_est = f_shrinkage(parameter,Lambda,tau); % x_a_est = x_a(t+1)
            t = t + 1; 
            % particular stop condition
            if norm(x_a_est-x_a_old) < delta     
                Tmax = t;
                break;
            end
        end
        
        % CLEANING
        cleaning_threshold = 10*tau*lambda;
        % nullification of elements with module less than the threshold interval
        x_a_est(abs(x_a_est)<=cleaning_threshold) = 0;

        % j-th ITERATION RESULT EVALUATION
        x_est = x_a_est(1:n); % estimated state extraction
        a_est = x_a_est(n+1:q+n); % estimated attack extraction
        a_support = sort(a_support); % attack support
        a_support_est = find(a_est); % estimated attack support
        vettTmax(j) = Tmax; % total iterations vector
        % boolean attack support evaluation
        if size(a_support_est) == size(a_support)
            if a_support_est == a_support
                a_success_counter = a_success_counter + 1; % success counter
                a_estimation_results(j) = 1; % 1: right estimation, 0: wrong one
            end
        end
        % array of state mean squared errors (only correct estimations are considered)
        x_estimation_accuracy(j) = norm(x-x_est)/n_iter; 
    end
    
    % TASK 2 RESULTS
    a_success_percentage = a_success_counter/n_iter*100; % percentage of right attack support estimation
    fprintf('The attack was correctly detected %d times over %d simulations, so in %.2f %% of cases \n', a_success_counter, n_iter, a_success_percentage)
    fprintf('With an average mean squared error between real and estimated state equals to %.7f \n', mean(x_estimation_accuracy))
    
end