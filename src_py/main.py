import polars as pl
from experiments.genetic_algo import REAL_DATA_GA_EXPERIMENT

pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

if __name__ == "__main__":
    REAL_DATA_GA_EXPERIMENT.run()



