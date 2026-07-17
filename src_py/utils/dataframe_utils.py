import polars as pl
from consts import *
from sklearn.preprocessing import KBinsDiscretizer

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

def verify_columns(lf: pl.LazyFrame | pl.DataFrame, required_columns: list) -> None:
    """
    Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    Args:
        lf (pl.LazyFrame | pl.DataFrame): DataFrame a verificar
        
    Raises:
        ValueError: Si falta alguna columna necesaria
    """
    
    if isinstance(lf, pl.DataFrame):
        missing_columns = [col for col in required_columns if col not in lf.columns]
    else:
        missing_columns = [col for col in required_columns if col not in lf.collect_schema().names()]
    
    if missing_columns:
        raise ValueError(f"Las siguientes columnas no existen y son necesarias en el DataFrame: {missing_columns}")

def get_grouping_dataframe(df: pl.DataFrame) -> pl.DataFrame:
    """
    Obtiene las características de agrupamiento de los estudiantes

    Args:
        df (pl.DataFrame): DataFrame con las características de los estudiantes

    Returns:
        pl.DataFrame: DataFrame con las características de agrupamiento
    """
    
    #Verifica que el DataFrame tenga las columnas necesarias para la generación del DataFrame de agrupamiento    
    verify_columns(df, REQUIRED_HG_COLUMNS) 

    df = df.with_columns(
        AN = discretize_column(df["AN"], 5),
        RN = discretize_column(df["RN"], 5),
        CN = discretize_column(df["CN"], 5),
        BE = discretize_column(df["BE"], 5),
        EE = discretize_column(df["EE"], 5),
        CE = discretize_column(df["CE"], 5),
        HS = discretize_column(df["HS"], 5),
        PL = discretize_column(df["PL"], 5),
    )
    return df.select([
        "Cronotype",
        "AN",
        "RN",
        "CN",
        "BE",
        "EE",
        "CE",
        "HS",
        "PL",
        "VARK",
        "MI"
    ])