from genia_libs import GeneticAlgorithm, hypergraph_from_dataframe
from genia_libs.utils.dataframe import get_grouping_dataframe,load_preprocessed_lf
import matplotlib.pyplot as plt
import polars as pl

DATA_PATH = "data/test_data/Psychoeducational_Features_for_Group_Forming.parquet"
HYPERGRAPH_PATH = "data/test_data/PFGF1_hypergraph.hg"

    
if __name__ == "__main__":
    """_summary_
        
        Load the synthetic data of 400 students, create the hypergraph of features and perform the group formation process.
    - GA parameters:
    - Initial population: 30
    - Maximum generations: 300000
    - Spins per generation: 7
    - Elitism: 2
    - Mutation: 25% (Fixed)
    - Crossover: 95%
    - Number of groups to form: 16
    """

    ga = GeneticAlgorithm(30, 300000, 7, 2, 0.95, None) #Initial population, max generations, spins per generation, elitism, crossover probability, experiment file path
    ga.show_config()
    groups = ga.run(16, HYPERGRAPH_PATH) # Form 16 groups
        
    for group in groups:
        df = load_preprocessed_lf(DATA_PATH).collect() #Load the preprocessed data of 400 students
        group_df = df.filter(pl.col("Id").is_in(group))
        print(f"Grupo {groups.index(group) + 1}:")
        print(group_df)