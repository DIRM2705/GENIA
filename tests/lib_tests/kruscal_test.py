from group_enhancer import make_bias_matrix, kruscal_minimum_spanning_tree
import polars as pl
import pytest

def test_kruscal():
    # Crear un DataFrame de Polars con datos mixtos
    df = pl.DataFrame({
        "media": [10, 7.4, 6.8, 9.1, 8.3, 5.5, 7.9, 6.1, 8.7, 9.5],
    },
    schema={
        "media": pl.Float64,
    })

    # Calcular la matriz de sesgo
    bias_matrix = make_bias_matrix(df)
    # Verificar la forma de la matriz resultante
    assert len(bias_matrix) == 10
    
    kruscal = kruscal_minimum_spanning_tree(bias_matrix)
    del bias_matrix # Liberar memoria
    
    while (node := kruscal.next()) is not None:
        print(node)
