/// Calculate the homogeneity metric for a given set of probabilities.
/// 
/// # Arguments
/// 
/// * `probabilities` - A slice of probabilities representing the distribution of categories
///
/// # Returns
/// 
/// * `f64` - The homogeneity metric, where a value closer to 0 indicates a more homogeneous distribution
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
    return scale(probabilities.len() as f64, 1.0 - sum);
}

/// Calculate the balance metric for a given set of probabilities and possible outcomes.
/// 
/// # Arguments
///
/// * `probabilities` - A slice of probabilities representing the distribution of categories
/// * `possible_outcomes` - The number of possible outcomes in the distribution
/// # Returns
/// 
/// * `f64` - The balance metric, where a value closer to 0 indicates a more balanced distribution
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
    return scale(possible_outcomes, (perfect_balance - sum).abs());
}


// Scale a value to the range [0, 1] based on the number of categories
/// 
/// # Arguments
/// 
/// * `num_categories` - The number of categories in the distribution
/// * `value` - The value to be scaled
/// 
/// # Returns
/// * `f64` - The scaled value in the range [0, 1]
fn scale(num_categories : f64, value : f64) -> f64
{   
    return (num_categories * value)/(num_categories - 1.0);
}