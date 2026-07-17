#from genia_libs import GeneticAlgorithm  # TODO: Compile with maturin
from utils.hypergraph_utils import create_hipergraph
from utils.dataframe_utils import get_grouping_dataframe
import polars as pl

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
    HYPERGRAPH_PATH = "src_py/data/synthetic_chars.hg"
    CHARACTERISTICS_PATH = "src_py/data/synthetic_chars.parquet"
    
    df = get_grouping_dataframe(CHARACTERISTICS_PATH)
    create_hipergraph(HYPERGRAPH_PATH)
    ga = GeneticAlgorithm(100, 5000, 25, 2, 10, 50)
    best_groups = ga.run(16, HYPERGRAPH_PATH) # 8 grupos a formar
    _print_groups(df, best_groups)
        
if __name__ == "main":

    pl.Config.set_tbl_cols(-1)
    pl.Config.set_tbl_rows(-1)
    _synthetic_data_experiment()