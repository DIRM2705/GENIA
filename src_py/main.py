import numpy as np
import polars as pl
from preprocessing.nlp import *

CURRICULUM_MODEL = None

def cargar_modelo():
    global CURRICULUM_MODEL
    CURRICULUM_MODEL = pl.read_parquet(f"src_py/models/curriculm_vectors.parquet").lazy()

def _get_campo_vector(lema : str) -> np.ndarray:
    """_summary_
    Función que recibe una palabra y devuelve el vector semántico correspondiente
    Args:
        lema (str): Lema a buscar
    Returns:
        np.ndarray: Vector semántico correspondiente a la palabra
    """
    
    
    lema_df = CURRICULUM_MODEL.filter(pl.col("lema") == lema).collect()
    if lema_df.is_empty():
        return np.zeros(4)
    
    return lema_df.drop("lema").to_numpy()[0]

def clasificar_curriculum(documento : list[str]) -> int:
    """_summary_
    Función que recibe la lista de lemas de un temario y devuelve la clase correspondiente
    Args:
        documento (list[str]): Lemas del temario
    Returns:
        int : Clase correspondiente al curriculum
    """
    if CURRICULUM_MODEL is None:
        raise Exception("No se ha cargado un modelo de clasificación para el curriculum")

    vector = np.zeros(4)

    for lema in documento:
        vector += _get_campo_vector(lema)

    return np.argmax(vector)