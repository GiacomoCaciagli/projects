clc
clear all
close all

%% Variables initialization
delay = 0;
x0 = zeros(4,1);


Cf = 2*60000; % N/rad per tyre
Cr = 2*57000; % N/rad per tyre
mv = 1575; % kg
Vx = 80/3.6; % m/s
Jpsi = 2875; % kgm^2
lf = 1.3; % m front semi-wheelbase
lr = 1.5; % m semi-wheelbase
l = lr+lf; % total wheelbase
oversteering = Cf*lf-Cr*lr>0; 
understeering = Cf*lf-Cr*lr<0; 

ref = 1;
simtime=200;
if ref==1 && Vx~=80/3.6
    simtime=simtime*(80/3.6)/Vx;
end

if ref==2
    if Vx~= 80/3.6
        simtime=140*(80/3.6)/Vx;
    else
       simtime=140;
    end
end
if ref==3 || ref==4
    simtime=50;
end

% abs(delta)<=25 deg

A=[0 1 0 0;
    0 -(Cf+Cr)/(mv*Vx) (Cf+Cr)/mv (Cr*lr-Cf*lf)/(mv*Vx);
    0 0 0 1;
    0 (Cr*lr-Cf*lf)/(Jpsi*Vx) (Cf*lf-Cr*lr)/Jpsi -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx)];

B1=[0; Cf/mv; 0; (Cf*lf)/Jpsi];

B2=[0; ((Cr*lr-Cf*lf)/(mv*Vx))-Vx; 0; -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx)];

B=[B1 B2];

C=eye(4);

D=zeros(4,2);

%P=[-1 -2 -3 -4];
%K=place(A,B1,P);
Q = eye(4)*100;
R = eye(1)*100;
sys = ss(A,B1,C,zeros(4,1));
[K,S,P] = lqr(sys,Q,R);
Kff=((mv*Vx^2)/l)*((lr/Cf)-(lf/Cr)+(lf*K(3)/Cr))+l-(lr*K(3));


%%
close all;
refs = [1,2,3,4]
for i = 1:length(refs)
ref = refs(i)
simtime=200;
if ref==1 && Vx~=80/3.6
    simtime=simtime*(80/3.6)/Vx;
end

if ref==2
    if Vx~= 80/3.6
        simtime=140*(80/3.6)/Vx;
    else
       simtime=140;
    end
end
if ref==3 || ref==4
    simtime=50;
end

sim('model.slx')

figure(3)
plot(time,trajectory)
%title('Curvature trajectory')
ylabel('Kl [1/m]')
xlabel('Time [s]')
grid on

figure(4)
subplot(2,2,1)
plot(time,delta_des,time,beta,time,yaw_rate)
%title('delta input')
ylabel('Radiants [rad]')
xlabel('Time [s]')
legend("delta","sideslip","yaw rate")
legend('Location','southeast')
grid on



subplot(2,2,2)
plot(time,alfa_front,time,alfa_rear)
%title('Slip angles')
ylabel('Angle [°]')
xlabel('Time [s]')
legend('Front slip angle','Rear slip angle')
legend('Location','southeast')
grid on

subplot(2,2,3)
plot(time,lateral_deviation)
%title('Lateral error')
ylabel('Meters [m]')
xlabel('Time [s]')
grid on

subplot(2,2,4)
plot(time,heading_angle_error)
%title('Heading angle error')
ylabel('Radiants [rad]')
xlabel('Time [s]')
grid on
saveas(gcf,'images/all.png')
figure(1203)
plot(time,lateral_acceleration)
%title('Heading angle error')
ylabel('acceleration [m/s^2]')
xlabel('Time [s]')
grid on

figure(5)
plot(Var_trajectory(:,1),Var_trajectory(:,2),'LineWidth',3)
hold on
plot(x,y,'LineWidth',1)
%title('Trajectories')
ylabel('Meters [m]')
xlabel('Meters [m]')
legend('Vehicle trajectory','True trajectory')
grid on

end

%% integral control
x0 = zeros(4,1)
ref = 1;
simtime = 200;
sim('model.slx')
no_int = lateral_deviation;
%%
x0 = zeros(5,1)
A=[0 1 0 0 0;
    0 -(Cf+Cr)/(mv*Vx) (Cf+Cr)/mv (Cr*lr-Cf*lf)/(mv*Vx) 0;
    0 0 0 1 0;
    0 (Cr*lr-Cf*lf)/(Jpsi*Vx) (Cf*lf-Cr*lr)/Jpsi -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx) 0;
    1 0 0 0 0];

B1=[0; Cf/mv; 0; (Cf*lf)/Jpsi; 0];

B2=[0; ((Cr*lr-Cf*lf)/(mv*Vx))-Vx; 0; -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx); 0];

B=[B1 B2];

C=eye(5);

D=zeros(5,2);

Q = eye(5);
R = eye(1);
sys = ss(A,B1,C,zeros(5,1));
[K,S,P] = lqr(sys,Q,R);

Kff=((mv*Vx^2)/l)*((lr/Cf)-(lf/Cr)+(lf*K(3)/Cr))+l-(lr*K(3));

%% cambiare dimensione mux nel simulink !!!!!!!

sim('model.slx')
figure(6)
plot(time,lateral_deviation)
hold on
plot(time,no_int)
legend('integral effect','no integral effect')
xlabel('t [s]')
ylabel('$\Delta y$ [m]','Interpreter','latex')
grid on
%saveas(gcf,'images/integral_effect.png')

%% gain speed dependency
close all
k1 = [];
k2 = [];
k3 = [];
k4 = [];
kk = [];
kf = [];
v = linspace(1/3.6,80/3.6);
for i = 1:length(v)
Vx = v(i);
A_g=[0 1 0 0;
    0 -(Cf+Cr)/(mv*Vx) (Cf+Cr)/mv (Cr*lr-Cf*lf)/(mv*Vx);
    0 0 0 1;
    0 (Cr*lr-Cf*lf)/(Jpsi*Vx) (Cf*lf-Cr*lr)/Jpsi -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx)];

B1_g=[0; Cf/mv; 0; (Cf*lf)/Jpsi];

B2_g=[0; ((Cr*lr-Cf*lf)/(mv*Vx))-Vx; 0; -(Cr*lr^2+Cf*lf^2)/(Jpsi*Vx)];

B_g=[B1_g B2_g];

C_g=eye(4);

D_g=zeros(4,2);

Q = eye(4);
R = eye(1);
sys = ss(A_g,B1_g,C_g,zeros(4,1));
[K_g,S,P] = lqr(sys,Q,R);
Kff=((mv*Vx^2)/l)*((lr/Cf)-(lf/Cr)+(lf*K_g(3)/Cr))+l-(lr*K_g(3));

k1 = [k1;K_g(1)];
k2 = [k2;K_g(2)];
k3 = [k3;K_g(3)];
k4 = [k4;K_g(4)];
kk = [kk;norm(K_g)];
kf = [kf;Kff];
end
figure(1)
plot(v,k1)
xlabel('v [m/s]')
ylabel('Gain k(1)')
%saveas(gcf,'images/k1.png')
figure(2)
plot(v,k2)
xlabel('v [m/s]')
ylabel('Gain k(2)')
%saveas(gcf,'images/k2.png')
figure(3)
plot(v,k3)
xlabel('v [m/s]')
ylabel('Gain k(3)')
%saveas(gcf,'images/k3.png')
figure(4)
plot(v,k4)
xlabel('v [m/s]')
ylabel('Gain k(4)')
%saveas(gcf,'images/k4.png')
figure(6)
plot(v,kf)
xlabel('v [m/s]')
ylabel('Gain kff')
%saveas(gcf,'images/kf.png')

%% evaluation for different tunings (valutare per diversi ref?)
x0 = zeros(4,1);
refs = [1,2,3,4];
Qt = eye(4);
velocities = [80/3.6; 130/3.6];
Qd = {Qt*100;Qt*100;Qt*100};
Rd = [1,10,100];
RMSES = [];
ENERGY = [];
FW_E = [];
FB_E = [];
for j = 1:length(refs)
ref = refs(j);
for k = 1 : length(velocities)
Vx = velocities(k)

for i = 1:length(Rd)
    close all
    simtime=200;
if ref==1 && Vx~=80/3.6
    simtime=simtime*(80/3.6)/Vx;
end

if ref==2
    if Vx~= 80/3.6
        simtime=140*(80/3.6)/Vx;
    else
       simtime=140;
    end
end
if ref==3 || ref==4
    simtime=50;
end
    Q = Qd{i};
    R = Rd(i);
    [A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
    sim('model.slx');
    RMSE = sum((lateral_deviation.^2)/length(lateral_deviation));
    signal_energy = (energy(end));
    RMSES = [RMSES;RMSE];
    FW_E = [FW_E;fw_e(end)];
    FB_E = [FB_E;fb_e(end)];
end
end
end

%% feedback and feedforward contributions
x0 = zeros(4,1);
simtime = 200;
ref = 1;
Q = 100*eye(4);
R = 100*eye(1);
[A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
sim('model.slx')
plot(time,feedback)
hold on
plot(time,feedforward)
grid on
legend('feedback','feedforward')
legend('Location','northwest')
xlabel('t [s]')
%saveas(gcf,'images/feedback.png')
ylabel('angle [rad]')

%% OVER,UNDER AND NEUTRAL
x0 = zeros(4,1);
simtime = 200;
ref = 1;
Cf = 2*60000; 
Cr = 2*57000; 
Q = 100*eye(4);
R = 100*eye(1);
Vx = 80/3.6;
[A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
sim('model.slx')
delta_under = delta_des;
lat_under = lateral_deviation;

Cf = 2*60000; 
Cr = 2*57000/10; 
[A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
sim('model.slx')
delta_over = delta_des;
lat_over = lateral_deviation;

Cf = 2*60000; 
Cr = 2*60000; 
[A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
sim('model.slx')
delta_neutral = delta_des;
lat_neutral = lateral_deviation;

figure(20)
plot(time,delta_under)
hold on
plot(time,delta_over)
hold on
plot(time,delta_neutral)
grid on
xlabel('t [s]')
ylabel('angle [rad]')
legend('understeering','oversteering','neutral')
%saveas(gcf,'images/delta_underoverneutral.png')
figure(21)
plot(time,lat_under)
hold on
plot(time,lat_over)
hold on
plot(time,lat_neutral)
grid on
xlabel('t [s]')
ylabel('$\Delta y$ [m]','Interpreter','latex')
legend('understeering','oversteering','neutral')
%saveas(gcf,'images/lat_underoverneutral.png')

%% feedforward delay
delay = 0
Cf = 2*60000; 
Cr = 2*57000; 
Q = 100*eye(4);
R = 100*eye(1);
Vx = 80/3.6;
[A,B,C,D,K,Kff] = change_matrices(Vx,Cf,Cr,Q,R);
sim('model.slx')
lat_no = lateral_deviation;

delay = 1;
sim('model.slx')

%saveas(gcf,'images/delta_underoverneutral.png')
figure(23)
plot(time,lat_no)
hold on
plot(time,lateral_deviation)
grid on
xlabel('t [s]')
ylabel('$\Delta y$ [m]','Interpreter','latex')
legend('no delay','delay')
%saveas(gcf,'images/delay.png')