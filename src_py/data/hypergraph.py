from pathlib import Path
from genia_libs import hypergraph_from_dataframe
from data.dataframe import get_characteristics_dataframe

def create_hipergraph(hypergraph_path: Path):
    hypergraph_path = Path(hypergraph_path)
    if hypergraph_path.exists():
        return
    
    parquet_path = hypergraph_path.with_suffix(".parquet")
    df = get_characteristics_dataframe(parquet_path)
    
    # Crear el hipergrafo de características a partir del dataframe, excluyendo las columnas "Id" y "TND"
    hypergraph_from_dataframe(df)