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

def _print_groups(df : pl.DataFrame, best_groups : list[list[int]]):
    df = df.with_row_index("Id")
    for i, group in enumerate(best_groups):
        print(f"Grupo {i+1}:")
        group_df = df.filter(pl.col("Id").is_in(group))
        print(group_df)

def _real_data_experiment():
    """_summary_
    Experimento: Algoritmo Genético con datos Reales
    
    Este algoritmo genético carga el DataFrame con los 30 estudiantes reales
    crea el hipergrafo de características y realiza el proces de formación de grupos
    Parámetros del algoritmo genético:
    - Población: 100
    - Número de generaciones: 5000
    - Spins por generación: 25
    - Elitismo: 2
    - Mutación: 10%
    - Cruzamiento: 50%
    - Número de grupos a formar: 5
    """
    HYPERGRAPH_PATH = "src_py/data/characteristics.hg"
    CHARACTERISTICS_PATH = "src_py/data/characteristics.parquet"
    
    df = get_grouping_dataframe(CHARACTERISTICS_PATH)
    create_hipergraph(HYPERGRAPH_PATH)
    ga = GeneticAlgorithm(100, 5000, 25, 2, 10, 50)
    best_groups = ga.run(5, HYPERGRAPH_PATH) # 5 grupos a formar
    _print_groups(df, best_groups)
    
def _synthetic_data_experiment():
    """_summary_
    Experimento: Algoritmo genético con datos sintéticos
    
    Este algoritmo genético carga el DataFrame con los 399 estudiantes sintéticos
    crea el hipergrafo de características y realiza el proceso de formación de grupos
    Parámetros del algoritmo genético:
    - Población: 100
    - Número de generaciones: 5000
    - Spins por generación: 25
    - Elitismo: 2
    - Mutación: 10%
    - Cruzamiento: 50%
    - Número de grupos a formar: 8
    """
    HYPERGRAPH_PATH = "data/test_data/hypergraph_test.hg"
    ga = GeneticAlgorithm(8, 1500, 25, 2, 90, 70, "src_py/experiments/genetic_algorithm/experiment.txt")
    
    #start_time = time.time()
    for _ in range(200):
        ga.run(16, HYPERGRAPH_PATH) # 16 grupos a formar
    #end_time = time.time()
    #print(f"Execution time: {end_time - start_time} seconds")
        
if __name__ == "__main__":
    _synthetic_data_experiment()
    input("Press Enter to continue...")
    
    exp_id = -1
    with open("src_py/experiments/genetic_algorithm/experiment.txt", "r") as infile, open("src_py/experiments/genetic_algorithm/experiment_clean.txt", "w") as outfile:
        outfile.write("ID_experiment,Generation,Best_fitness,Converged\n")
        for line in infile.readlines():
            strip_line = line.strip()
            if not strip_line:
                continue
            
            if strip_line.startswith("Pobl"):
                exp_id += 1
                continue
                    
            if strip_line.endswith("True"):
                outfile.write(f"{exp_id},{strip_line}\n")
            else:
                outfile.write(f"{exp_id},{strip_line},False\n")
    
    pl.Config.set_tbl_cols(-1)
    pl.Config.set_tbl_rows(-1)
    
    
    df = pl.read_csv("src_py/experiments/genetic_algorithm/experiment_clean.txt").lazy()#Get data from all experiments
    df = df.with_columns(pl.col("Best_fitness").round(4))
    experiment_results = df.unique("ID_experiment", keep="last").sort("ID_experiment") #Final output of the GA
    
    # Calculate mean executed generations and best fitness
    means = experiment_results.select("Generation", "Best_fitness").mean().collect()
    print("Mean executed generations before convergence: ", means["Generation"][0])
    print("Mean best fitness: ", means["Best_fitness"][0])
    
    #Analyze fitness values to find max and min fitness and the number of experiments that fall near those values
    fitness_analysis = experiment_results.select("ID_experiment", "Best_fitness")
    
    max_fitness_reached = fitness_analysis.select("Best_fitness").max().collect()["Best_fitness"][0]
    experiments_with_max_fitness = fitness_analysis.filter(pl.col("Best_fitness").is_close(max_fitness_reached, abs_tol=1e-2)).collect()
    print("Max fitness: ", max_fitness_reached)
    print("Experiments near max fitness:")
    print(experiments_with_max_fitness)
    
    min_fitness_reached = fitness_analysis.select("Best_fitness").min().collect()["Best_fitness"][0]
    experiments_with_min_fitness = fitness_analysis.filter(pl.col("Best_fitness").is_close(min_fitness_reached, abs_tol=1e-2)).collect()
    print("Min fitness: ", min_fitness_reached)
    print("Experiments near min fitness:")
    print(experiments_with_min_fitness)
    
    # Analyze convergence behavior
    autoconvergence_df = df.group_by("ID_experiment").agg([pl.col("Converged").any().alias("Autoconverged")])
    convergence_df = (df
                   .unique(subset=["ID_experiment", "Best_fitness"], keep="first", maintain_order=True)
                   .group_by("ID_experiment")
                   .agg([
                       pl.col("Best_fitness").max().alias("Max Fitness"),
                       pl.col("Generation").last().alias("Converge Generation"),
                   ])
                   .join(autoconvergence_df, on="ID_experiment", how="left")
                   .sort("ID_experiment")
    )
    
    autoconverged_df = convergence_df.filter((pl.col("Autoconverged") == True)).select(["ID_experiment", "Max Fitness", "Converge Generation"])
    maxed_iter_df = convergence_df.filter((pl.col("Autoconverged") == False)).select(["ID_experiment", "Max Fitness", "Converge Generation"])
    
    convergence_df = convergence_df.collect()
    
    print("Convergence analysis:")
    print(convergence_df)
    
    mean_convergence_generation = convergence_df.select("Converge Generation").mean()["Converge Generation"][0]
    print("Mean convergence generation: ", mean_convergence_generation)
    
    mean_autoconverged_count = autoconverged_df.collect().height
    mean_maxed_iter_count = maxed_iter_df.collect().height
    
    print("Number of experiments that autoconverged: ", mean_autoconverged_count)
    print("Number of experiments that maxed iterations: ", mean_maxed_iter_count)
    
    autoconvergence_fitness_fun = autoconverged_df.select("Converge Generation", "Max Fitness").sort("Converge Generation").collect()
    
    autoconverged_df = autoconverged_df.with_columns(
        min = pl.col("Max Fitness").min(),
        q1 = pl.col("Max Fitness").quantile(0.25),
        median = pl.col("Max Fitness").median(),
        q3 = pl.col("Max Fitness").quantile(0.75),
        max = pl.col("Max Fitness").max(),
        mean = pl.col("Max Fitness").mean(),
        std = pl.col("Max Fitness").std(),
        iqr = (pl.col("Max Fitness").quantile(0.75) - pl.col("Max Fitness").quantile(0.25))
    ).first().select(["min", "q1", "median", "q3", "max", "mean", "std", "iqr"])
    
    maxed_iter_df = maxed_iter_df.with_columns(
        min = pl.col("Max Fitness").min(),
        q1 = pl.col("Max Fitness").quantile(0.25),
        median = pl.col("Max Fitness").median(),
        q3 = pl.col("Max Fitness").quantile(0.75),
        max = pl.col("Max Fitness").max(),
        mean = pl.col("Max Fitness").mean(),
        std = pl.col("Max Fitness").std(),
        iqr = (pl.col("Max Fitness").quantile(0.75) - pl.col("Max Fitness").quantile(0.25))
    ).first().select(["min", "q1", "median", "q3", "max", "mean", "std", "iqr"])

    autoconverged_df = autoconverged_df.collect()
    maxed_iter_df = maxed_iter_df.collect()
    
    print("Autoconverged statistics:")
    print(autoconverged_df)
    
    print("Maxed iterations statistics:")
    print(maxed_iter_df)

    print("Autoconvergence convergence-fitness function:")
    print(autoconvergence_fitness_fun)

    #_synthetic_data_experiment()