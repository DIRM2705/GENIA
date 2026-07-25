import sys
import os

# Get the absolute path to the parent directory
parent_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '../..'))
sys.path.append(parent_dir)

from genia_libs import GeneticAlgorithm
from main import create_hipergraph, load_preprocessed_lf
from utils.dataframe_utils import get_grouping_dataframe
import polars as pl
import time



"""
Experimentos con el algoritmo genético carga el DataFrame de los estudiantes,
crea el hipergrafo de características y realiza el proceso de formación de grupos usando 
el algoritmo genético, mostrando los grupos formados al final

Configuración del algoritmo genético (Parámetros a ajustar):
- Población: Número de soluciones (conjuntos de grupos) en la población inicial
- Número de generaciones: Cuántas veces se repetirá el proceso de selección, cruzamiento
    y mutación.
- Spins por generación: Cuántas veces se girará la ruleta en cada generación, se generan 4 soluciones por spin.
- Elitismo: Número de las mejores soluciones que se mantienen sin cambios en la siguiente generación
- Mutación: Probabilidad de que una solución sufra cambios aleatorios para introducir diversidad.
- Cruzamiento: Porcentaje aproximado de alumnos que se intercambiarán entre dos soluciones
"""

EXPERIMENT_FILE_PATH = "src_py/experiments/genetic_algorithm/experiment_12000.txt"
    
def _synthetic_data_experiment():
    """_summary_
    Experimento: Algoritmo genético con datos sintéticos
    
    Este algoritmo genético carga el DataFrame con los 399 estudiantes sintéticos
    crea el hipergrafo de características y realiza el proceso de formación de grupos
    Parámetros del algoritmo genético:
    - Población: 8
    - Número de generaciones: 12000
    - Spins por generación: 2
    - Elitismo: 2
    - Mutación: 90%
    - Cruzamiento: 70%
    - Número de grupos a formar: 16
    """
    HYPERGRAPH_PATH = "data/test_data/hypergraph_test.hg"
    ga = GeneticAlgorithm(30, 300000, 7, 2, 0.25, 0.95, EXPERIMENT_FILE_PATH) # 30 poblacion, 300000 generaciones, 7 spins, 2 elitismo, 0.25 mutacion, 0.95 cruzamiento
    ga.show_config()
    for _ in range(10):
        print(f"Running experiment {_+1}/10")
        ga.run(16, HYPERGRAPH_PATH) # 16 grupos a formar
        
if __name__ == "__main__":
    RESULTS_FILE_PATH = "src_py/experiments/genetic_algorithm/results_12000.csv"
    TIMES_FILE_PATH = "src_py/experiments/genetic_algorithm/times_12000.csv"
    #_synthetic_data_experiment()
    #input("Press Enter to continue...")
    
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
                                                