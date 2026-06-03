import polars as pl
from pathlib import Path
from preprocessing.dataframe import discretize_column

def get_dataframe(parquet_path: str) -> pl.DataFrame:
    parquet_path = Path(parquet_path)
    if not parquet_path.exists():
        df = _get_dataframe_from_csv(parquet_path.with_suffix(".csv"))
        df.write_parquet(parquet_path)
    else:  
        df = pl.read_parquet(parquet_path)
    return df

def _get_dataframe_from_csv(csv_path: Path) -> pl.DataFrame:
    if not csv_path.exists():
        raise FileNotFoundError(f"El archivo {csv_path} no existe.")
    
    return pl.read_csv(csv_path)
    
def get_characteristics_dataframe(characteristics_path: str) -> pl.DataFrame:
    df = get_dataframe(characteristics_path)
    df = df.with_columns(
        AM = discretize_column(df["AM"], 5),
        RM = discretize_column(df["RM"], 5),
        CM = discretize_column(df["CM"], 5),
        BE = discretize_column(df["BE"], 5),
        EE = discretize_column(df["EE"], 5),
        CE = discretize_column(df["CE"], 5)
    )
    return df.select(pl.exclude("Id", "TND"))