from group_enhancer import make_bias_matrix
import numpy as np
import polars as pl
import pytest

def test_bias_matrix():
    # Crear un DataFrame de Polars con datos mixtos
    df = pl.DataFrame({
        "media": [10, 7.4, 6.8, 9.1, 8.3, 5.5, 7.9, 6.1, 8.7, 9.5],
    },
    schema={
        "media": pl.Float64,
    })

    # Calcular la matriz de Gower
    bias_matrix = make_bias_matrix(df)
    # Verificar la forma de la matriz resultante
    assert len(bias_matrix) == 10

    # Verificar algunos valores específicos en la matriz de Gower
    assert np.isclose(bias_matrix.get(0, 1), 0.77)
    assert np.isclose(bias_matrix.get(1, 2), 0.83)
    assert np.isclose(bias_matrix.get(3, 4), 0.77)
    assert np.isclose(bias_matrix.get(5, 6), 1.23)
    assert np.isclose(bias_matrix.get(7, 8), 0.53)
    assert np.isclose(bias_matrix.get(8, 9), 1.17)