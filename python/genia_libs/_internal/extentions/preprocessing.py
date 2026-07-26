from sklearn.preprocessing import KBinsDiscretizer
import polars as pl

def discretize_column(column: pl.Series, n_bins: int) -> pl.Series:
    """
    Discretize a continuous column into n_bins using KBinsDiscretizer from sklearn.

    Args:
        column (pl.Series): The continuous column to be discretized.
        n_bins (int): The number of bins to discretize into.

    Returns:
        pl.Series: A new Polars Series with the discretized values.
    """
    # Convertir la columna de Polars a un array de numpy para usar con KBinsDiscretizer
    column_np = column.to_numpy().reshape(-1, 1) #reshape para convertirlo en una matriz de una sola columna, que es lo que espera KBinsDiscretizer
    
    # Crear el discretizador con n_bins y estrategia de cuantiles para que cada bin tenga aproximadamente la misma cantidad de muestras
    discretizer = KBinsDiscretizer(n_bins=n_bins, encode='ordinal', strategy='kmeans')
    
    # Ajustar el discretizador a los datos y transformarlos
    discretized_np = discretizer.fit_transform(column_np).astype(int).flatten() #fit_transform para ajustar el modelo y transformar los datos, astype(int) para convertir los valores a enteros, flatten() para convertir la matriz resultante en un array unidimensional
    
    # Convertir el array de numpy resultante de nuevo a una Serie de Polars
    discretized_series = pl.Series(discretized_np, dtype=pl.UInt8) #Convertimos a UInt8 para ahorrar espacio, ya que el número de bins es pequeño
    
    return discretized_series 