import polars as pl
from pathlib import Path

def setup_experiment(hypergraph_path: Path):
    pl.Config.set_tbl_cols(-1)
    pl.Config.set_tbl_rows(-1)