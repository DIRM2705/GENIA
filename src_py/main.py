import polars as pl 
from core.preprocess import load_from_csv
from core.consts import *
from pathlib import Path
from genia_libs import hypergraph_from_dataframe, GeneticAlgorithm

# [DEBUG] Configuración para visualizar el DataFrame completo 
pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

hypergraph_path = Path("characteristics.hg")
if not hypergraph_path.exists():
    df = load_from_csv("Pruebas1.csv") #cargar el archivo csv con los datos de los estudiantes usando la función de carga de datos, que devuelve un DataFrame de Polars
    print(df) #imprimir el DataFrame para verificar que se haya procesado correctamente
    hypergraph_from_dataframe(df.select(pl.exclude("Id", "TND"))) #crear un hipergráfico a partir del DataFrame usando la función de creación de hipergráficos, que toma el DataFrame y la ruta donde se guardará el hipergráfico como argumentos

# Configuración del algoritmo genético:
# - Población
# - Número de generaciones
# - Spins por generación
# - Elitismo
# - Mutación
# - Cruzamiento
ga = GeneticAlgorithm(100, 500, 25, 2, 10, 50)
best_groups = ga.run(5) # 5 grupos a formar

for i, group in enumerate(best_groups):
    for student in group:
        print(f"Grupo {i+1}: {student}")
