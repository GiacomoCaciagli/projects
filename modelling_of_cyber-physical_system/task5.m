%% TASK 5: Distributed estimation
clc
clear all
close all

% CONSTANTS AND PARAMETERS
matrices = open('data/stochasticmatrices.mat'); % stochastic matrices load
n = 10; % state dimension
q = 20; % sensors
h = 2; % sensor attacks
sigma = 1e-2; % Gaussian standard deviation of the noise
eta = sigma*randn(1); % Gaussian noise
tau = 0.03; % learning rate
lambda = (2*1e-4)/tau; % sparsity coefficient
Lambda = lambda*[zeros(n,1);ones(q,1)]; % sparsity array
max_iter = 20; % number of runs of DIST algorithm for each matrix Q
Tmax = 1e5; % maximum number of DIST algorithm iterations
delta = 1e-7; % stop criterion threshold

avg_convergence_time = zeros(1,4); % average DIST convergence time over all iterations for each Q
avg_tot_a_est_success_perc = zeros(1,4); % average correctly detected attack percentage for each Q
avg_tot_x_est_accuracies = zeros(1,4); % average state estimation accuracy over all iterations for each Q
avg_consensus_deviation = zeros(1,4); % average mean variance of the state estimations for each Q
consensus_counter = zeros(1,4); % number of times in which the consensus is achieved for each topology Q

% FOR EACH MATRIX 
for matrix = 1:4
    % over all iterations for each Q
    tot_a_est_success = 0; % correct attack estimations 
    tot_a_est_success_perc = 0; % estimation accuracy percentage
    avg_support_a_est = 0; % average consensus of the estimated attacks
    tot_x_est_accuracy = zeros(20,1); % state estimation accuracy array
    convergence_times = zeros(20,1); % convergence time array for all iterations
    consensus_deviation = zeros(20,1); % mean variance between the distributed state estimations for each iteration
   
    fprintf('\n%d) Q%d stochastic matrix\n', matrix, matrix)
    switch matrix
        case 1
            Q = matrices.Q1;
        case 2
            Q = matrices.Q2;
        case 3
            Q = matrices.Q3;
        case 4
            Q = matrices.Q4;
    end
    Q_eigenvalues = sort(eig(Q),'descend'); % sorted eigenvalues of Q
    Q_ESR(matrix) = Q_eigenvalues(2); % essential spectral radius of Q
    fprintf('The essentials spectral radius of Q%d is '+string(Q_ESR(matrix))+'\n',matrix)
    f = figure('Name', strcat('Q',string(matrix)));
    network_graph = digraph(Q);
    plot(network_graph);
    drawnow

    for n_iter = 1:max_iter
        
        iter_a_est_success = 0; % correct attack estimation counter of the iteration
        iter_x_est_accuracy = 0; % mean squared error between real and estimated state
        % unaware attack generation
        a = zeros(q,1); % real attack
        for i = 1:h
            % support random generation   
            support_a(i) = randi(q);
            while ismember(support_a(i), support_a(1:i-1,1))
                support_a(i) = randi(q);                  
            end 
            % element random generation in [-2,-1]U[1,2] interval
            a(support_a(i)) = 1 + 2*rand(1);
            a(support_a(i),a(support_a(i))>2) = a(support_a(i)) - 4;
        end
    
        % DISTRIBUTED ITERATIVE SHRINKAGE THRESHOLDING ALGORITHM
        x_tilde = randn(n,1); % consensual state estimation to achieve
        C = randn(q,n); % output matrix
        G = [C eye(q)]; % output matrix of the attacked measurements (partial lasso)
        y = C*x_tilde + eta + a; % real output measurements with noise and attacks
        z_old = zeros(n+q,q); % z(k) matrix of current local estimated states and attacks by all sensors (each column is a node)
        z_est = zeros(n+q,q); % z(k+1) matrix of next local estimated states and attacks by all sensors
        parameter = 0; % to be passed to the shrinkage-operator
    
        for k = 1:Tmax
            z_old = z_est; % z_est(k)
            for i = 1:q
                % parameter computation
                parameter = (Q(i,:)*z_est')' + tau*G(i,:)'*(y(i)-G(i,:)*z_est(:,i)); % agent i computes a weighted mean of its received neighbors states which is added to the error between output and estimated values
                % shrinkage-operator call
                z_est(:,i) = f_shrinkage(parameter,Lambda,tau); % z_est_i(k+1)
            end
            % sum of consecutive local estimations mean squared errors
            tot_MSE = sum(vecnorm(z_est-z_old));
            % particular stop condition
            if tot_MSE < delta
                convergence_times(n_iter) = k;
                consensus_counter(matrix) = consensus_counter(matrix) + 1;
                break;
            end
        end

        x_est = z_est(1:n,:); % local estimated states
        a_est = z_est(n+1:n+q,:); % local estimated attacks
       
        % ATTACK MATRIX CLEANING
        cleaning_threshold = 0.2;
        % nullification of attacks with module within the threshold interval
        a_est(abs(a_est)<=cleaning_threshold) = 0;

        % ESTIMATED ATTACK SUPPORT EVALUATION
        support_a = find(a); % real attack support
        % comparison between real and estimated attack support for each node
        for node = 1:q
            support_a_est = find(a_est(:,node)); % estimated attack support of a single node
            iter_a_est_success = iter_a_est_success + sum(ismember(support_a,support_a_est))/length(support_a); % cumulative sum of right estimations over an iteration, adding the fraction of attacked sensors correctly estimated by a node
        end
        tot_a_est_success = tot_a_est_success + iter_a_est_success; % cumulative estimation success over all iterations
        
        % ESTIMATED STATE EVALUATION
        x_bar = mean(x_est,2); % average state computed taking the mean of the estimated state rows 
        iter_x_est_accuracy = norm(x_tilde-x_bar)^2; % mean squared error between real and average estimated state
        tot_x_est_accuracy(n_iter) = iter_x_est_accuracy; % update of the iteration state estimation accuracy array
        consensus_deviation(n_iter) = mean(var(x_est,1,2)); % computation of the mean variance normalized to n of the agents state estimates for each node (along rows)

        % SINGLE ITERATION RESULTS
        avg_support_a_est = mean(support_a_est,2); % average consensus on the attacked sensors
        iter_a_est_success_percentage = iter_a_est_success*100/q ;% percentage of correct estimations by all nodes in a single iteration  
        iter_x_est_accuracy; % mean squared error between the real and the average estimated state of a single interation

    end

    % Q-MATRIX RESULTS
    tot_a_est_success_perc = tot_a_est_success*100/(q*20);
    tot_x_est_accuracy_perc = mean(tot_x_est_accuracy);
    avg_convergence_time(matrix) = mean(convergence_times);
    avg_tot_a_est_success_perc(matrix) = tot_a_est_success_perc;
    avg_tot_x_est_accuracies(matrix) = mean(tot_x_est_accuracy);
    avg_consensus_deviation(matrix) = mean(consensus_deviation);
end

% TASK 5 RESULTS
fprintf('\nTask 5: Distributed Estimation \n')
Q_ESR % essential spectral radius array for each Q
avg_convergence_time % average DIST convergence time over all iterations for each Q
avg_tot_a_est_success_perc % average correctly detected attack percentage for each Q
avg_tot_x_est_accuracies % average estimation accuracy over all iterations for each Q
avg_consensus_deviation % average variance of the agent state estimates for each Q
consensus_counter % number of times in which the consensus is achieved for each topology Q