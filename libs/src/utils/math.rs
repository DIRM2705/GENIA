pub fn gini_index(probabilities : &[f64]) -> f64
{
    let mut sum = 0.0;
    for p in probabilities {
        sum += p * p;
    }
    return 1.0 - sum;
}

pub fn perfect_balance_param(group_size : f64) -> f64
{
    return (group_size - 1.0) / group_size;
}