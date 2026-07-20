from pathlib import Path
import polars as pl
from utils.dataframe_utils import verify_columns
from preprocessing.graficas import export_info_from_console
from consts import REQUIRED_INPUT_COLUMNS, REQUIRED_OUTPUT_COLUMNS
from genia_libs import hypergraph_from_dataframe
from utils.dataframe_utils import get_grouping_dataframe

def lazy_from_csv(file_path : Path) -> pl.LazyFrame:
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
    
    #Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    verify_columns(lf, REQUIRED_INPUT_COLUMNS) 
    
    return lf

def load_preprocessed_lf(parquet_path : Path) -> pl.LazyFrame:
    """
    Cargar un dataframe previamente procesado

    Args:
        parquet_path (Path): La ruta del archivo parquet que contiene el dataframe previamente procesado

    Raises:
        ValueError: Si el archivo no es un archivo parquet
        FileNotFoundError: Si el archivo no existe

    Returns:
        pl.LazyFrame: El lazyframe de polars cargado desde el archivo parquet
    """
    if not parquet_path.suffix == ".parquet":
        raise ValueError(f"El archivo {parquet_path.absolute()} no es un archivo parquet")
    if not parquet_path.exists():
        raise FileNotFoundError(f"El archivo {parquet_path.absolute()} no existe")
     
    lf = pl.read_parquet(parquet_path).lazy()
    
    #Verifica que el DataFrame tenga las columnas de salida del preprocesamiento
    verify_columns(lf, REQUIRED_OUTPUT_COLUMNS) 
    return lf

def create_hipergraph(df: pl.DataFrame, hypergraph_path: Path) -> None:
    if hypergraph_path.exists():
        return
    
    # Crear el hipergrafo de características a partir del dataframe, excluyendo las columnas "Id"
    hypergraph_from_dataframe(df, str(hypergraph_path))