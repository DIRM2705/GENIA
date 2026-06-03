import polars as pl

def setup_experiment():
    pl.Config.set_tbl_cols(-1)
    pl.Config.set_tbl_rows(-1)