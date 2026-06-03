from genia_libs import GeneticAlgorithm
from utils.dataframe_utils import get_characteristics_dataframe
from experiments.experiment import Experiment
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

def _real_data_experiment():
    ga = GeneticAlgorithm(100, 500, 25, 2, 10, 50)
    best_groups = ga.run(5) # 5 grupos a formar
    _print_groups(best_groups)
    
def _print_groups(best_groups : list[list[int]]):
    df = get_characteristics_dataframe("characteristics.parquet")
    df = df.with_row_index("Id")
    for i, group in enumerate(best_groups):
        print(f"Grupo {i+1}:")
        group_df = df.filter(pl.col("Id").is_in(group))
        print(group_df)
        
REAL_DATA_GA_EXPERIMENT = Experiment(
    name = "Algoritmo Genético con Datos Reales",
    explanation = """Este algoritmo genético carga el DataFrame con los 30 estudiantes reales
    crea el hipergrafo de características y realiza el proces de formación de grupos
    Parámetros del algoritmo genético:
    - Población: 100
    - Número de generaciones: 500
    - Spins por generación: 25
    - Elitismo: 2
    - Mutación: 10%
    - Cruzamiento: 50%
    - Número de grupos a formar: 5""",
    run_function = _real_data_experiment
)