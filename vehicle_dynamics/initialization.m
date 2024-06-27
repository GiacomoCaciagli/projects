clc
clear all
close all

%% PAC
% Load tyre data (Pacejka model coefficients)
% File with the tyre data
WheelFile = 'Tyre215_50_19_Comb';      
% Wheel Initialization
pacn = [];
eval(['[Pacejka]=' WheelFile ';'])
pacn = struct2cell(Pacejka);
for ii = 1:size(pacn)
    Pace(ii) = pacn{ii};
end
Pacn = Pace'; 

%% variable inizialization

m = 1812; % kg, mass of the vehicle
Af = 2.36; % m^2, frontal area of the car
Cx = 0.27; % aerodynamic drag coefficient
rho = 1.225; % kg/m^3, air density
velstart = 0.5;
%velstart = 40/3.6;
rpm = 16000;
g = 9.81; % m/s^2
Fz = m*g/2; % N, vertical force
W = 215; % mm, tyre width
AR = 50; % aspect ratio
D = 19*25.4; % mm, rim diameter
H = (AR*W)/100; % mm, tyre side wall height
R = (D+2*H)/(2*1000); % m, tyre radius 
h = 0.55;
eta = 0.90; % efficiency of the electric motor
phi = 0.95; % efficiency of the mechanical transmission
Pmax = 150000; % W
Cm = 310; % Nm, maximum torque
f0 = 0.009;
f1 = 6.5e-6; % s^2/m^2;
Ir = 1; % wheel mass moment inertia
L = 2.77; % wheel base
Gr = 10.5; % open differential
pedal = 1;
Fx_lim = m*30; % brake force max
brakes = 1;
mu = 0.5;
regTorque = -0.1*g*R*m/(Gr); % regenerative torque
ABS_ON = 0;
brake_start = 10;
TCS_ON = 1;
EBD_ON = 0;
battery_capacity = 58*3.6*10^6;
battery_limit = 99;
sim_time = 50;