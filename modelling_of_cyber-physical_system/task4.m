%% Task 4: Sparse observer
clc
close all
clear all
format default

% CONSTANTS, PARAMETERS AND RESULTS INITIALIZATION
delete('results/Task4/Task4_SparseObserver.txt') 
load('data/task4data.mat');
load('data/task4_sensors_positions.mat');
j = 4; % targets
p = 100; % cells
q = 25; % sensors
h = 2; % attacks
I = eye(q); % attack output matrix
G = [D I]; % output matrix of the attacked measurements (partial lasso)
G_n = normalize(G); % normalized output matrix
sigma = 0.2; % standard deviation of the Gaussian noise
eta = sigma*randn(q,1); % Gaussian noise
epsilon = 1e-8; % error on learning rate
tau = norm(G_n, 2)^-2 - epsilon; % learning rate
Lambda = [10*ones(p,1);20*ones(q,1)]; % sparsity array
T = 50; % maximum evolution time of the simulation (min 50)

attack_description = ["Unaware time-invariant attack", "Aware time-varying attack"];
localization_result_comparison = zeros(2,T); % comparision of the two localization results for each instant
attack_estimation_comparison = zeros(2,T); % comparison of the two attack estimation for each instant

% REAL TARGETS POSITIONS AND ATTACKS
x_true = zeros(p,1); % x(i): real inital target position in one of p cells (0: absent, 1: present)
a_true = zeros(q,1); % a(i): real attack on one of q sensors
support_x_true = zeros(j,1); % real initial targets positions support
support_a_true = zeros(h,1); % real attacks support

% real targets positions generation
for i = 1:j
   support_x_true(i) = randi(p);
   while ismember(support_x_true(i), support_x_true(1:i-1))
        support_x_true(i) = randi(p); % target support random generation                    
   end
   x_true(support_x_true(i)) = 1;
end

% attacks support random generation (constant over time)
for i = 1:h
    support_a_true(i) = randi(q);
    while ismember(support_a_true(i), support_a_true(1:i-1,1))
        support_a_true(i) = randi(q);                   
    end
end

% SCENARIO = 1: UNAWARE ATTACK, 2: AWARE ATTACK
for scenario = 1:2
    x = x_true; % inital targets positions
    a = a_true; % real attack
    support_x = support_x_true; % inital targets positions support
    support_a = support_a_true; % real attacks support
    
    % INITIAL VISUAL REPRESENTATION
    f = figure('Name', attack_description(scenario));
    f.WindowState = 'maximized';
    f.Position(3:4)=[800 400];
    hold on
    grid on
    plot_sensors = scatter(sensors_pos(1,:),sensors_pos(2,:),100,[0 0.4470 0.7410],'filled');
    plot_targets = scatter(nan,nan,300,'yellow','filled','square');
    plot_estimated_targets = scatter(nan,nan,300,'green','square','LineWidth',2.5);
    plot_attacked_sensors = scatter(nan,nan,100,[0.7650 0.0780 0.1840],'filled');
    plot_estimated_attacked_sensors = scatter(nan,nan,100,'green','LineWidth',2.5);
    legend('sensors','targets','estimated targets','attacked sensors','estimated attacked sensors','Location','eastoutside');
    
    plot_targets.XData = (mod(find(x)-1,10))*100+50;
    plot_targets.YData = (9-floor((find(x)-1)/10))*100+50;

    % attack support visualization
    plot_attacked_sensors.XData = sensors_pos(1,support_a)';
    plot_attacked_sensors.YData = sensors_pos(2,support_a)';

    % TARGETS POSITIONS AND ATTACKS ONLINE ESTIMATION 
    x_est = zeros(p,1); % estimated target position
    a_est = zeros(q,1); % estimated attack
    z_est = [x_est;a_est]; % estimated target position and attack initially zero
    
    
    localization_success_results = zeros(1,T); % array of targets positions estimation outcomes
    localization_success_counter = 0; % number of correctly estimated targets positions 
    localization_success_percentage = 0; % percentage of correctly estimated target position
    attack_estimation_results = zeros(1,T); % array of attack estimation outcomes
    attack_estimation_counter = 0; % number of correctly estimated attacks
    attack_estimation_percentage = 0; % percentage of correctly estimated attacks

    for k = 1:T
        % REAL
        y = D*x + eta; % y(k) real output measurements with noise
        % real attacks generation
        for i = 1:h
            switch scenario
                case 1
                    a(support_a(i)) = 30; % constant attack on sensor i
                case 2
                    a(support_a(i)) = 0.5*y(support_a(i)); % proportional attack on sensor i
            end
        end
        y_a = y + a; % y(k) real output measurements with noise and attacks
        x = A*x; % x(k+1) = A*x(k) real system dynamics

        % ESTIMATE
        z_old = z_est; % z(k) estimated state and attack
        parameter = z_old + tau*G_n'*(y_a-G_n*z_old); % to be passed to the IST
        z_est = f_shrinkage(parameter,Lambda,tau); % z(k+1) next estimated state and attack
        x_est = A*z_est(1:p); % x_est(k+1) next estimated state
        a_est = z_est(p+1:p+q); % a_est(k+1) next estimated attack
        z_est = [x_est;a_est]; % z_est(k+1) next estimated state and attack
    
        % TARGETS POSITIONS CLEANING
        % j maximum RSS extraction from the estimated state
        [x_est_max x_est_max_indexes] = maxk(x_est, j); 
        x_est(x_est_max_indexes) = 1; % unitary substitution of the j highest values
        x_est(~ismember(1:p, x_est_max_indexes)) = 0; % zero substitution of the other values
    
        % ATTACKS ARRAY CLEANING
        cleaning_threshold = 2;
        % nullification of elements with module less than the threshold interval
        a_est(abs(a_est)<cleaning_threshold) = 0;
        % h maximum values extraction from the attack
        [a_est_max, support_a_est_max] = maxk(abs(a_est), h);
        % zero substitution of q-h minimum values using a mask
        a_est(~ismember([1:q], support_a_est_max)) = 0; 
    
        % k-INSTANT RESULTS
        k;
        x_vs_x_est = [x x_est];
        a_vs_a_est = [a a_est];

        % localization support evaluation
        support_x = find(x); % real targets positions support
        support_x_est = find(x_est); % estimated targets positions support 
        localization_success_results(k) = sum(support_x==support_x_est)/j; % fraction of targets correctly localized
        localization_success_counter = localization_success_counter + sum(support_x==support_x_est); % number of correctly estimated targets

        % attack support evaluation
        support_a = find(a); % real attack support
        support_a_est = find(a_est); % estimated attack support 
        attack_estimation_results(k) = sum(numel(intersect(support_a, support_a_est)))/h; % fraction of attacks correctly localized
        attack_estimation_counter = attack_estimation_counter + sum(numel(intersect(support_a, support_a_est))); % number of correctly estimated attack

        % ESTIMATED TARGETS POSITONS AND ATTACKS VISUALIZATION
        title("Real and estimated targets positions and attacked sensors at instant k = " + k);
        plot_estimated_attacked_sensors.XData = sensors_pos(1,(a_est~=0))';
        plot_estimated_attacked_sensors.YData = sensors_pos(2,(a_est~=0))';
        plot_targets.XData = (mod(find(x)-1,10))*100+50;
        plot_targets.YData = (9-floor((find(x)-1)/10))*100+50;
        plot_estimated_targets.XData = (mod(find(x_est)-1,10))*100+50;
        plot_estimated_targets.YData = (9-floor((find(x_est)-1)/10))*100+50;
        pause(0.5); % simulate refresh rate of 0.5 second 

    end

    % INSTANCE STATISTICS
    fprintf('\n\n\nTask 4: Sparse observer - case: %d) ', scenario)
    switch scenario
        case 1
            fprintf('Unaware time-invariant attack: a(i) = 30 for each sensor under attack\n')
        case 2
            fprintf('Aware time-varying attack: the attacker takes the sensor measurement y(i) and corrupts it of a quantity equal to 0.5 y(i)\n')
    end
    localization_result_comparison(scenario,:) = localization_success_results;
    attack_estimation_comparison(scenario,:) = attack_estimation_results;
    localization_success_percentage = localization_success_counter/(j*T)*100
    attack_estimation_percentage = attack_estimation_counter/(h*T)*100

end

% OVERALL RESULTS BETWEEN THE TWO SCENARIOS
localization_result_comparison; % comparision of the two localization results for each instant
attack_estimation_comparison; % comparison of the two attack estimation for each instant