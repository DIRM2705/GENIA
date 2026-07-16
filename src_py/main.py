from pathlib import Path
import sys
import polars as pl
from experiments.genetic_algo import REAL_DATA_GA_EXPERIMENT
from preprocessing.dataframe import load_from_csv
from preprocessing.graficas import export_info_from_console

pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

if __name__ == "__main__":
    df = load_from_csv("Pruebas1.csv")
    export_info_from_console(df)
    #REAL_DATA_GA_EXPERIMENT.run()



