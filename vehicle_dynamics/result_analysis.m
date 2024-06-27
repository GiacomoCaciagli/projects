warning('off','all')
%% result analysis

%% acceleration tests

% velstart 0.5 this times are obtained with mu=0.9 and stopping the
% simulation when the velocity is reached
% 2.6660 0->40 km/h  pedal=1
% 3.9190 0->60 km/h  pedal=1
% 5.6130 0->80 km/h  pedal=1
% 7.4950 0->60 mph ~ 97 km/h  pedal=1
% 7.8740 0->100 km/h  pedal=1
% 19.8350 0->100 mph ~ 160 km/h  pedal=1
% speed after 50 seconds = 57.8706 m/s

%% longitudinal acceleration

velstart = 0.5;
ABS_ON = 1;
TCS_ON = 1;
EBD_ON = 1;
sim_time = 50;
brake_start = 50;
pedal = 0.9;
mu = 0.9;

sim('model.slx')
figure(1)
plot(t,a)
%title('Longitudinal acceleration')
ylabel('Acceleration [m/s^2]')
xlabel('Time [s]')
grid on

% mu = 0.9 e pedal = 0.9
% c'è traction control

%%
figure(2)
plot(t,aerodynamic_drag)
%title('Aerodynamic drag force')
ylabel('Force [N]')
xlabel('Time [s]')
grid on

%%
figure(3)
plot(t,rolling_resistance)
%title('Rolling resistance')
ylabel('Force [N]')
xlabel('Time [s]')
grid on

%%
figure(4)
plot(t,electric_powertrain)
%title('Electric powertrain')
ylabel('Torque [Nm]')
xlabel('Time [s]')
grid on

%%
figure(5)
plot(t,requested_torque,t,torque_wheel)
%title('Requested motor Torque')
ylabel('Torque [Nm]')
xlabel('Time [s]')
legend('requested torque','output torque')
grid on

%%
figure(6)
plot(t,transmission_power_loss)
%title('Requested motor Torque')
ylabel('Power [W]')
xlabel('Time [s]')
grid on

%%
figure(7)
rear_tyre_slip_power_loss = rear_Fx.*rear_slip_ratio.*vel;
front_tyre_slip_power_loss = front_Fx.*front_slip_ratio.*vel;

plot(t,rear_tyre_slip_power_loss,t,front_tyre_slip_power_loss,t,rear_tyre_slip_power_loss+front_tyre_slip_power_loss)
%title('Requested motor Torque')
ylabel('Power [W]')
xlabel('Time [s]')
legend('rear axle tyres power loss','front axle tyres power loss','total power loss')
grid on

%%
% the power consumption value is obtained setting the motor torque equal to
% the resistances of the rear tyres and simulating for 10 s
cons_second = power_consumption(2000); % J/s

max_t = battery_capacity/cons_second;

total_travel_distance = velstart*max_t/1000 % in km

cons = (cons_second/3600)*(1000/velstart)

% media a 110 km/h 379
% 40 km/h -> 802.9026  cons second 2.8895e+03 J/s o 72.2379 Wh/Km
% 60 km/h -> 594.6456  cons second 5.8522e+03 J/s o 97.5371 Wh/Km
% 80 km/h -> 436.4232  cons second 1.0632e+04 J/s o 132.8985 Wh/Km
% 110 km/h -> 283.4951  cons second 2.2505e+04 J/s o 204.5891 Wh/Km

%% tip in tip off test
velstart = 1;
TCS_ON = 0;
ABS_ON = 0;
EBD_ON = 0;
sim_time = 5;
brake_start = 2.5;
brakes = 0;
pedal = 1;

sim('model.slx')
figure(8)
plot(t,torque_wheel)
ylabel('Motor Torque [Nm]')
xlabel('Time [s]')
grid on

figure(9)
plot(t,a)
ylabel('acceleration [m/s^2]')
xlabel('Time [s]')
grid on

%% regenerative breaking
mu = 0.9;
sim_time = 80;
brake_start = 50;
TCS_ON = 1;
ABS_ON = 1;
EBD_ON = 0;
brakes = 0;
pedal = 1;
velstart = 0.5;

sim('model.slx')
v_nobrake = vel;
bc_nobrake = battery_charge;
t_nobrake = t;

%%
%this may crash at low velocities, commented out so the flow of execution is not interrupted
%%
%{
brakes = 0.25;
sim('model.slx')
v_quarter_brake = vel;
bc_quarter_brake = battery_charge;
t_quarter_brake = t;
%}
%%
brakes = 0.5;
sim('model.slx')
v_half_brake = vel;
bc_half_brake = battery_charge;
t_half_brake = t;
%%
figure(10)
plot(t_nobrake,bc_nobrake)
hold on
%plot(t_quarter_brake,bc_quarter_brake)
%hold on
plot(t_half_brake,bc_half_brake)
ylabel('Energy [J]')
xlabel('Time [s]')
grid on

figure(11)
plot(t_nobrake,v_nobrake)
hold on
%plot(t_quarter_brake,v_quarter_brake)
%hold on
plot(t_half_brake,v_half_brake)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on

%% emergency braking
velstart = 30;
sim_time = 30;
brake_start = 10;
pedal = 1;
brakes = 1;
ABS_ON = 0;
TCS_ON = 0;
EBD_ON = 0;
pedal = 1;
mu = 0.9

% dry with no abs
sim('model.slx')
figure(12)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean acceleration
index = find(t == brake_start)
stop_distance_dry = abs(vel(index)^2/(2*mean(a(index:end-1))))

% dry with abs
ABS_ON = 1;
sim('model.slx')
figure(13)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean deceleration
index = find(t == brake_start)
stop_distance_dry_abs = abs(vel(index)^2/(2*mean(a(index:end-1))))

%dry with EBD
EBD_ON = 1;
sim('model.slx')
figure(14)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean acceleration
index = find(t == brake_start)
stop_distance_dry_ebd = abs(vel(index)^2/(2*mean(a(index:end-1))))

mu = 0.5;
% wet with no abs
ABS_ON = 0;
EBD_ON = 0;
sim('model.slx')
figure(15)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean acceleration
index = find(t == brake_start)
stop_distance_wet = abs(vel(index)^2/(2*mean(a(index:end-1))))

% wet with abs
ABS_ON = 1;
sim('model.slx')
figure(16)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean acceleration
index = find(t == brake_start)
stop_distance_wet_abs = abs(vel(index)^2/(2*mean(a(index:end-1))))

%wet with EBD
EBD_ON = 1;
sim('model.slx')
figure(17)
plot(t,vel)
hold on
plot(t,v_wheel)
ylabel('velocity [m/s]')
xlabel('Time [s]')
grid on
% the stopping distance is computed by dividing the square of the velocity
% by the mean acceleration
index = find(t == brake_start)
stop_distance_wet_ebd = abs(vel(index)^2/(2*mean(a(index:end-1))))