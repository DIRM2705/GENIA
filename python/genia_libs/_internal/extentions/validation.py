import polars as pl


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