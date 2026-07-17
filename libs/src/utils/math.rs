pub fn homogeneity_metric(probabilities : &[f64]) -> f64
{
    /*
     * Gini's impurity index measures the heterogeneity of a group,
     * as it approaches 0, the group is more homogeneous
     */

    let mut sum = 0.0;
    for p in probabilities {
        sum += p * p;
    }
    return 1.0 - sum;
}

pub fn balance_metric(probabilities : &[f64], possible_outcomes : f64) -> f64
{
    /*
    * The balance metric measures the distance to a perfectly balanced distribution,
    *  as it approaches 0, the group is more balanced
    */

    let perfect_balance = 1.0 / possible_outcomes;
    let mut sum = 0.0;
    for p in probabilities {
        sum += p * p;
    }
    return (perfect_balance - sum).abs();
}