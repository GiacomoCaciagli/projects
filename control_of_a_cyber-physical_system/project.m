clc
clear all
close all

A = [0 1; 880.87 0]; %Matrice A iniziale
B = [0; -9.9453]; %Matrice B iniziale
C =[708.27 0]; %Matrice C iniziale
D_ = zeros(2,1)'; %Matrice D iniziale

q = 100;
r = 20;
Q = q*eye(2); %Weighting matrix of input
R = r; %Weighting matrix of state

X0 = [1/C(1) 1/800]'; % initial conditions
c = 5; % calculated so that it is greater than the minimal coupling gain of every topology
T_fin = 100;
noise = 0;%set to 1 if there's noise
for i=1:1 %i = 1 costante; i = 2 rampa; i = 3  sin(t); i = 4 sin(3t);

    switch i
        case 1
            k_leader = place(A,B,[0 -1]);
        case 2
            k_leader = acker(A,B,[0 0]);
        case 3
            k_leader = acker(A,B,[1*sqrt(-1) -1*sqrt(-1)]);
        case 4
            k_leader = acker(A,B,[3*sqrt(-1) -3*sqrt(-1)]);
    end
    A0 = A;
    A = A-B*k_leader; %viene imposto il comportamento a tutti gli agenti
    P = are(A, B*inv(R)*B',Q); %soluzione di riccati
    K = inv(R)*B'*P; %control gain
    P2 = are(A',C'*inv(R)*C,Q); %seconda soluzione di riccati
    F = P2*C'*inv(R); %distributed observer gain
    
    L_leader = acker(A',C',[-1,-2])'; %obsverver del leader

    for j=1:5 %j = 1 complete graph; j = 2 binary tree; j = 3 star graph; j = 4 circular graph; j = 5 random graph
    
        switch j
            case 1
                %complete graph
                Ad = ones(6);
                Ad = Ad-diag([1 1 1 1 1 1]);
                D = diag([5 5 5 5 5 5]); % sum of the weights of the incoming links
                G = diag([1 1 1 1 1 1]);
            case 2
                %binary tree graph
                Ad = [0 0 0 0 0 0; 0 0 0 0 0 0; 1 0 0 0 0 0; 0 1 0 0 0 0; 0 1 0 0 0 0; 0 0 0 0 1 0];
                D = diag([0 0 1 1 1 1]); % sum of the weights of the incoming links
                G = diag([1 1 0 0 0 0]); % links between the leader and the agents
            case 3
                % star graph (with the leader in the center, with monodirectional links) 
                Ad = [0 0 0 0 0 0; 0 0 0 0 0 0; 0 0 0 0 0 0; 0 0 0 0 0 0; 0 0 0 0 0 0; 0 0 0 0 0 0];
                D = diag([0 0 0 0 0 0]);
                %Ad = [zeros(1,6); D];
                G = diag([1 1 1 1 1 1]);
            case 4
                % circular graph (with monodirectional links)
                Ad = [0 0 0 0 0 1; 1 0 0 0 0 0; 0 1 0 0 0 0; 0 0 1 0 0 0; 0 0 0 1 0 0; 0 0 0 0 1 0];
                D = diag([1 1 1 1 1 1]);
                G = diag([1 0 0 0 0 0]);
            case 5
                %random graph
                Ad = [0 0 1 0 0 0; 1 0 0 0 0 0;0 1 0 0 0 0; 1 0 0 0 0 0; 0 0 0 0 0 0; 0 0 1 0 0 0];
                D = diag([1 1 1 1 0 1]);
                G = diag([1 0 0 0 1 0]);            
        end
        U = zeros(7,7);
        U(1,2:7) = [diag(G)']; 
        U(2:7,2:7) = U(2:7,2:7)+Ad';
        graph = digraph(U); 
        f1=figure(1);
        plot(graph) %plot del grafo
        L = D - Ad;
        Li{j} = L+G;
        lam_g = eig(L+G); % eigenvalues of the graph
        cc = 1/(2*min(real(lam_g))); %calcolo del minimo coupling gain
        F2  = place(A',-c*C',[-1 -2])'; %local observer gain %change to [-0.1 -0.2] in case if noise
        Ac{j} = kron(eye(6),A)-kron(c*(L+G),B*K);%matrice Ac
        
        sim("distributed.slx");
        
        sim("local.slx");


        
        clear displacement_error_distributed
        clear  displacement_error_local
        for t = 2:7 % we have to bring the error to 0
            displacement_error_distributed(:,t-1) = distributed.data(:,1)-distributed.data(:,t);%distributed displacement error for every agent
            displacement_error_local(:,t-1) = local.data(:,1)-local.data(:,t);%local displacement error for every agent
            
        end

        if noise == 0
            %convergence time as the time the follower always rests between
            %+-0.001
            %from the leader, valid only if there isn't noise
            for u = 1:length(displacement_error_distributed(1,:))
                tc = 0;
                for m = 1:length(displacement_error_distributed(:,1))
                    if abs(displacement_error_distributed(m,u)) <=abs(1e-3) 
                        if tc == 0
                            tc = distributed.time(m);
                        end
                    else
                        tc = 0;
                    end
                end
                conv_times_dist(j,u) = tc;
            end
    
            for u = 1:length(displacement_error_local(1,:))
                tc = 0;
                for m = 1:length(displacement_error_local(:,1))
                    if abs(displacement_error_local(m,u)) <= abs(1e-3) 
                        if tc == 0
                            tc = local.time(m);
                        end
                    else
                        tc = 0;
                    end
                end
                conv_times_loc(j,u) = tc;
            end
            %tempo di convergenza per ogni topologia calcolato come il tempo in cui
            %tutti i nodi sono in convergenza
            convergence_dist(j) = max(conv_times_dist(j,:));
            convergence_loc(j) = max(conv_times_loc(j,:));
            %visto che senza noise tutte le topologie vanno a circa lo stesso
            %steady-state, dobbiamo valutarne il comportamento a transitorio
            dist_interval = find(distributed.time >= convergence_dist(j),1) - 1;
            loc_interval = find(local.time >= convergence_loc(j),1) - 1;
            RMSE_dist(:,j) = mean(vecnorm(displacement_error_distributed(1:dist_interval,:),2,1)/sqrt(dist_interval)); %RMSE dal t=0 a t = tc
            RMSE_loc(:,j) = mean(vecnorm(displacement_error_local(1:loc_interval,:),2,1)/sqrt(loc_interval));%RMSE dal t=0 a t = tc


        end
    dis_end_dist(j,:) = displacement_error_distributed(end,:); %displacement errror a stedy-state(circa 0 senza rumore)
    dis_end_loc(j,:) = displacement_error_local(end,:);%displacement errror a stedy-state(circa 0 senza rumore)
    %close(f1)
    %plot del confronto per ogni agent tra displacement error local e distribuito
    switch j
        case 1 
            figure('Name','Complete Graph');
        case 2
            figure('Name','Binary Tree Graph');
        case 3
            figure('Name','Star Graph');
        case 4
            figure('Name','Circular Graph');
        case 5
            figure('Name','Random Graph');
    end
  
    for b = 1:6 
        subplot(2,3,b)
        grid, hold on
        plot(distributed.time,displacement_error_distributed(:,b))
        plot(local.time,displacement_error_local(:,b))
        legend('Distributed','Local')
        title('Displacement error on follower '+string(b)) 
    end
       
    end
    close(f1)
    %visualizzazione risultati
    if noise == 0
        convergence_dist %colonne = topologie
        convergence_loc%colonne = topologie
        RMSE_dist%colonne = topologie
        RMSE_loc%colonne = topologie
    end
    dis_end_dist %righe=topologie, colonne = agent
    dis_end_loc %righe=topologie, colonne = agent
end

