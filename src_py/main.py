from pathlib import Path
import sys
import polars as pl
from preprocessing.graficas import export_info_from_console

pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

if __name__ == "__main__":
    df = pl.read_parquet(Path("src_py/data/synthetic_chars.parquet"))
    export_info_from_console(df)
    #REAL_DATA_GA_EXPERIMENT.run()



