function shrinkage_operator = f_shrinkage(x_est,lambda,tau)

for i = 1:length(x_est)
    if x_est(i) > tau*lambda(i)
        x_est(i) = x_est(i) - tau*lambda(i);
    elseif x_est(i) < -tau*lambda(i)
        x_est(i) = x_est(i) + tau*lambda(i);
    elseif abs(x_est(i)) <= tau*lambda(i)
        x_est(i) = 0;
    end
end

shrinkage_operator = x_est;

end