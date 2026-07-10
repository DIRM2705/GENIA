import polars as pl
from pathlib import Path
from preprocessing.dataframe import discretize_column, grade_IM_scores, grade_VARK_scores

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
    df = pl.read_csv(file_path, infer_schema_length=1000) #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas
    
    #Procesar VARK
    VARK_scores = grade_VARK_scores(students.select([f"VARK{i}" for i in range(1,14)]))
    
    #PROCESAR IM
    IM_scores = grade_IM_scores(students.select(["IM1", "IM2", "IM3"]))

    #Agregar VARK al DataFrame de estudiantes
    students = students.hstack(VARK_scores) #hstack=horizontal stack -> Agrega columnas lado a lado -> agregar las columnas de VARK_scores al DataFrame de estudiantes
    
    #Agregar IM al DataFrame de estudiantes
    students = students.hstack(IM_scores) #Agrega las columnas de IM_scores al DataFrame de estudiantes
    
    return students #devuelve el DataFrame final   
    
def get_grouping_dataframe(dataframe_path: str) -> pl.DataFrame:
    """
    Obtiene las características de agrupamiento de los estudiantes

    Args:
        dataframe_path (str): Ruta al dataframe de características previamente preprocesado

    Returns:
        pl.DataFrame: DataFrame con las características de agrupamiento
    """
    try:
        df = load_preprocessed_df(dataframe_path)
    except FileNotFoundError:
        print("Se requiere preprocesar los datos para obtener el dataframe de caracteristicas")
        return None
        
    df = df.with_columns(
        AM = discretize_column(df["AM"], 5),
        RM = discretize_column(df["RM"], 5),
        CM = discretize_column(df["CM"], 5),
        BE = discretize_column(df["BE"], 5),
        EE = discretize_column(df["EE"], 5),
        CE = discretize_column(df["CE"], 5)
    )
    return df.select(pl.exclude("Id"))