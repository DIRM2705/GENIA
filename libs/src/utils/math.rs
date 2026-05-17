pub fn homogeneity_metric(probabilities : &[f64]) -> f64
{
    // El índice de impureza de Gini mide la heterogeneidad de un grupo,
    // a medida que se acerca a 0, el grupo es más homogéneo
    let mut sum = 0.0;
    for p in probabilities {
        sum += p * p;
    }
    return 1.0 - sum;
}

pub fn balance_metric(probabilities : &[f64], possible_outcomes : f64) -> f64
{
    // La métrica de balance mide la distancia a una distribución perfectamente equilibrada,
    // a medida que se acerca a 0, el grupo es más equilibrado

    let perfect_balance = 1.0 / possible_outcomes;
    let mut sum = 0.0;
    for p in probabilities {
        sum += p * p;
    }
    return (perfect_balance - sum).abs();
}