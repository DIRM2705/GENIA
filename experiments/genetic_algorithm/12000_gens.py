from genia_libs import GeneticAlgorithm, hypergraph_from_dataframe
from genia_libs.utils.dataframe import get_grouping_dataframe,load_preprocessed_lf
import polars as pl

EXPERIMENT_FILE_PATH = "src_py/experiments/genetic_algorithm/results/experiment_12000.txt"
DATA_PATH = "data/test_data/Psychoeducational_Features_for_Group_Forming.parquet"
HYPERGRAPH_PATH = "data/test_data/PFGF1_hypergraph.hg"

def _create_hypergraph():
    """_summary_

    Load the synthetic data of 400 students and create the hypergraph of features.
    """
    
    df = load_preprocessed_lf(DATA_PATH).collect() #Load the preprocessed data of 400 students
    grouping_df = get_grouping_dataframe(df)
    hypergraph_from_dataframe(grouping_df, HYPERGRAPH_PATH)
    
def _synthetic_data_experiment():
    """_summary_

    Load the synthetic data of 400 students, create the hypergraph of features
    and perform the group formation process.
    GA parameters:
    - Initial population: 30
    - Maximum generations: 300000
    - Spins per generation: 7
    - Elitism: 2
    - Mutation: 25% (Fixed)
    - Crossover: 95%
    - Number of groups to form: 16
    """

    ga = GeneticAlgorithm(30, 300000, 7, 2, 0.95, None) 
    ga.show_config()
    for test in range(10):
        print(f"Running test {test+1}/10")
        ga.run(16, HYPERGRAPH_PATH) # Form 16 groups
        
if __name__ == "__main__":
    RESULTS_FILE_PATH = "src_py/experiments/genetic_algorithm/results/results_12000.csv"
    TIMES_FILE_PATH = "src_py/experiments/genetic_algorithm/results/times_12000.csv"
    #_create_hypergraph()
    _synthetic_data_experiment()
    input("Press Enter to continue...")
    
    #exp_id = -1
    #with open(EXPERIMENT_FILE_PATH, "r") as infile, open(RESULTS_FILE_PATH, "w") as outfile, open(TIMES_FILE_PATH, "w") as timefile:
    #    outfile.write("ID_experiment,Generation,Best_fitness,Convergence_ratio\n")
    #    timefile.write("ID_experiment,Execution_seconds\n")
    #    for line in infile.readlines():
    #        strip_line = line.strip()
    #        if not strip_line:
    #            continue
    #        
    #        if strip_line.startswith("Pobl"):
    #            exp_id += 1
    #        elif strip_line.startswith("Sol"):
    #            timefile.write(f"{exp_id},{strip_line.split(":")[-1].replace(" segundos", "").strip()}\n")
    #        else:    
    #            outfile.write(f"{exp_id},{strip_line}\n")
    
    pl.Config.set_tbl_cols(-1)
    pl.Config.set_tbl_rows(-1)
    
    df = pl.read_csv(RESULTS_FILE_PATH).lazy()#Get data from all experiments
    df = df.with_columns(pl.col("Best_fitness").round(4))
    time_df = pl.read_csv(TIMES_FILE_PATH).lazy()#Get execution times from all experiments
    
    experiment_results = df.unique("ID_experiment", keep="last").sort("ID_experiment") #Final output of the GA
    experiment_results = experiment_results.join(time_df, on="ID_experiment", how="inner")
    
    print("Experiment results: ")
    print(experiment_results.collect())
    
    initial_fitness_df = df.unique("ID_experiment", keep="first").sort("ID_experiment").select(["ID_experiment", "Best_fitness"]).rename({"Best_fitness": "Initial_fitness"}) #Initial fitness of the GA
    mean_initial_fitness = initial_fitness_df.select("Initial_fitness").mean().collect()
    
    # Calculate mean executed generations and best fitness
    means = experiment_results.select("Generation", "Best_fitness").mean().collect()
    print("Mean executed generations before convergence: ", means["Generation"][0])
    print("Mean execution time: ", time_df.select("Execution_seconds").mean().collect()["Execution_seconds"][0], "seconds")
    #Analyze fitness values to find max and min fitness and the number of experiments that fall near those values
    fitness_analysis = experiment_results.select("ID_experiment", "Best_fitness")
    
    fitness_increment_df =(fitness_analysis.join(initial_fitness_df, on="ID_experiment", how="inner")
                           .with_columns(
                               Absolute_increment = (pl.col("Best_fitness") - pl.col("Initial_fitness")).alias("Fitness_increment"),
                               Relative_increment = (pl.col("Best_fitness") - pl.col("Initial_fitness")) / pl.col("Initial_fitness")
                           )
                           .sort("ID_experiment"))
    
    print("Fitness increment analysis:")
    print(fitness_increment_df.collect())
    
    print("Mean best fitness: ", means["Best_fitness"][0])
    print("Mean initial fitness: ", mean_initial_fitness["Initial_fitness"][0])
    
    max_fitness_reached = fitness_analysis.select("Best_fitness").max().collect()["Best_fitness"][0]
    print("Max final fitness: ", max_fitness_reached)
        
    min_fitness_reached = fitness_analysis.select("Best_fitness").min().collect()["Best_fitness"][0]
    print("Min final fitness: ", min_fitness_reached)
    
    mean_increment = fitness_increment_df.select("Relative_increment").mean().collect()["Relative_increment"][0]
    print("Mean relative fitness increment: ", mean_increment * 100, "%")
                                                