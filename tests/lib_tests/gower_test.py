from gower_distance import make_gower_matrix
import numpy as np
import polars as pl
import pytest

def test_gower_matrix_polars():
    engagement_levels = ["Muy motivado", "Motivado", "Neutro", "Poco Motivado", "Desmotivado"]
    # Crear un DataFrame de Polars con datos mixtos
    df = pl.DataFrame({
        "Linguistic": [1.0, 0.8, 0.3],
        "Logical": [0.5, 0.7, 0.2],
        "Musical": [0.2, 0.4, 0.9],
        "Bodily": [0.9, 0.6, 0.1],
        "Intrapersonal": [0.3, 0.5, 0.8],
        "Interpersonal": [0.4, 0.2, 0.7],
        "Naturalist": [0.6, 0.9, 0.4],
        "Visual": [0.7, 0.1, 0.5],
        "Aural": [0.8, 0.3, 0.6],
        "Read_Write": [0.2, 0.4, 0.9],
        "Kinesthetic": [0.5, 0.7, 0.2],
        "Behavioral": ["Motivado", "Neutro", "Desmotivado"],
        "Emotional": ["Motivado", "Desmotivado", "Neutro"],
        "Cognitive": ["Desmotivado", "Motivado", "Desmotivado"]
    },
    schema={
        "Linguistic": pl.Float64,
        "Logical": pl.Float64,
        "Musical": pl.Float64,
        "Bodily": pl.Float64,
        "Intrapersonal": pl.Float64,
        "Interpersonal": pl.Float64,
        "Naturalist": pl.Float64,
        "Visual": pl.Float64,
        "Aural": pl.Float64,
        "Read_Write": pl.Float64,
        "Kinesthetic": pl.Float64,
        "Behavioral": pl.Categorical,
        "Emotional": pl.Categorical,
        "Cognitive": pl.Categorical
    })

    # Calcular la matriz de Gower
    gower_matrix = make_gower_matrix(df)
    # Verificar la forma de la matriz resultante
    assert gower_matrix.size == 3
    
    for i in range(3):
        for j in range(3):
            assert 0.0 <= gower_matrix.get(i, j) <= 1.0

    # Verificar algunos valores específicos en la matriz de Gower
    assert np.isclose(gower_matrix.get(0, 0), 0.0)  # Distancia consigo mismo
    assert np.isclose(gower_matrix.get(0, 1), 0.602295)  # Distancia entre filas 0 y 1
    assert np.isclose(gower_matrix.get(1, 0), 0.602295)  # Distancia entre filas 0 y 1
    assert np.isclose(gower_matrix.get(0, 2), 0.70953)  # Distancia entre filas 0 y 2