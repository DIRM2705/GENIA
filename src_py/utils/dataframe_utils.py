import polars as pl
from pathlib import Path
from consts import *
from preprocessing.dataframe import preprocess
from sklearn.preprocessing import KBinsDiscretizer


def load_preprocessed_df(parquet_path : Path) -> pl.DataFrame:
    if not parquet_path.exists():
        raise FileNotFoundError(f"El archivo {parquet_path.absolute()} no existe")
     
    df = pl.read_parquet(parquet_path)
    return df

def preprocess_from_csv(file_path : Path) -> pl.DataFrame:
    """
    Dado un archivo en formato csv, crea un dataframe de polars creando las columnas necesarias
    
    Args:
        file_path (str): La ruta de un archivo csv con las siguientes columnas:
            - "ID": Identificador único del estudiante
            - "Cronotype": El cronotipo del estudiante
            - "AN": Porcentaje de satisfacción de la necesidad de autonomía
            - "RN": Porcentaje de satisfacción de la necesidad de relaciones
            - "CN": Porcentaje de satisfacción de la necesidad de competencia
            - "BE": Porcentaje de compromiso conductual
            - "EE": Porcentaje de compromiso emocional
            - "CE": Porcentaje de compromiso cognitivo
            - "HS": Porcentaje de búsqueda de ayuda
            - "PL": Porcentaje de aprendizaje por pares
            - "TM": Porcentaje de manejo del tiempo
            - "RH": Porcentaje de repetición
            - "EL": Porcentaje de elaboración
            - "OR": Porcentaje de organización
            - "CP": Porcentaje de pensamiento crítico
            - "MC": Porcentaje de metacognición
            - "MIKin": Puntaje de la inteligencia múltiple cinestésica
            - "MIExis": Puntaje de la inteligencia múltiple existencial
            - "MIInter": Puntaje de la inteligencia múltiple interpersonal
            - "MIIntra": Puntaje de la inteligencia múltiple intrapersonal
            - "MILog": Puntaje de la inteligencia múltiple lógico-matemática
            - "MIMus": Puntaje de la inteligencia múltiple musical
            - "MINat": Puntaje de la inteligencia múltiple naturalista
            - "MIVer": Puntaje de la inteligencia múltiple verbal
            - "MIVis": Puntaje de la inteligencia múltiple visual
            - "VARKVisual": Puntaje del estilo de aprendizaje visual
            - "VARKAural": Puntaje del estilo de aprendizaje auditivo
            - "VARKReadWrite": Puntaje del estilo de aprendizaje lectura/escritura
            - "VARKKinesthetic": Puntaje del estilo de aprendizaje kinestésico
            
    Returns:
        DataFrame: Un dataframe de polars con el formato requerido
    """
    
    if not file_path.exists():
        raise FileNotFoundError(f"El archivo {file_path.absolute()} no existe")
    
    lf = pl.read_csv(file_path, infer_schema_length=1000).lazy() #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas
    
    return preprocess(lf)

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
    
def get_grouping_dataframe(dataframe_path: str) -> pl.DataFrame:
    """
    Obtiene las características de agrupamiento de los estudiantes

    Args:
        dataframe_path (str): Ruta al dataframe de características previamente preprocesado

    Returns:
        pl.DataFrame: DataFrame con las características de agrupamiento
    """
    df = load_preprocessed_df(dataframe_path)
    
    df = df.with_columns(
        AN = discretize_column(df["AN"], 5),
        RN = discretize_column(df["RN"], 5),
        CN = discretize_column(df["CN"], 5),
        BE = discretize_column(df["BE"], 5),
        EE = discretize_column(df["EE"], 5),
        CE = discretize_column(df["CE"], 5),
        HS = discretize_column(df["HS"], 5),
        PI = discretize_column(df["PL"], 5),
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