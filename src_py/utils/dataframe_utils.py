import polars as pl
from pathlib import Path
from preprocessing.dataframe import discretize_column, grade_IM_scores, grade_VARK_scores
from consts import *

def load_preprocessed_df(parquet_path : Path) -> pl.DataFrame:
    if not parquet_path.exists():
        raise FileNotFoundError(f"El archivo {parquet_path.absolute()} no existe")
     
    df = pl.read_parquet(parquet_path)
    return df

def preprocess_from_csv(file_path : str, save_path : str) -> pl.DataFrame:
    """
    Dado un dataframe en formato csv, crea un dataframe de polars creando las columnas necesarias
    
    Args:
        file_path (str): La ruta de un archivo csv con las siguientes columnas:
            - "ID": Identificador único del estudiante
            - "Cronotype": El cronotipo del estudiante
            
    Returns:
        DataFrame: Un dataframe de polars con el formato requerido
    """
    
    #Leer csv con polars, y hacer el DataFrame
    students = pl.read_csv(file_path, infer_schema_length=1000) #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas
    
    verify_columns(students) #Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    #Procesar VARK
    VARK_scores = grade_VARK_scores(students.select(VARK_COLUMNS)) #Selecciona las columnas de VARK y las procesa para obtener los puntajes de VARK
    
    #PROCESAR IM
    IM_scores = grade_IM_scores(students.select(MI_COLUMNS)) #Selecciona las columnas de IM y las procesa para obtener los puntajes de IM

    #Agregar VARK al DataFrame de estudiantes
    students = students.hstack(VARK_scores) #hstack=horizontal stack -> Agrega columnas lado a lado -> agregar las columnas de VARK_scores al DataFrame de estudiantes
    
    #Agregar IM al DataFrame de estudiantes
    students = students.hstack(IM_scores) #Agrega las columnas de IM_scores al DataFrame de estudiantes
    
    return students #devuelve el DataFrame final   

def verify_columns(df: pl.DataFrame):
    """
    Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    Args:
        df (pl.DataFrame): DataFrame a verificar
        
    Raises:
        ValueError: Si falta alguna columna necesaria
    """
    
    missing_columns = [col for col in REQUIRED_COLUMNS if col not in df.columns]
    
    if missing_columns:
        raise ValueError(f"Las siguientes columnas no existen y son necesarias en el DataFrame: {missing_columns}")
    
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
        PI = discretize_column(df["PI"], 5),
    )
    return df.select(pl.exclude("Id"))