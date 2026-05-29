import polars as pl
from preprocessing.dataframe import discretize_column, load_from_csv
from pathlib import Path
from genia_libs import hypergraph_from_dataframe, GeneticAlgorithm

# [DEBUG] Configuración para visualizar el DataFrame completo 
pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

hypergraph_path = Path("characteristics.hg")
if not hypergraph_path.exists():
    parquet_path = Path("characteristics.parquet")
    if parquet_path.exists():
        df = pl.read_parquet(parquet_path) #cargar el DataFrame desde el archivo parquet si existe, para evitar tener que procesar el CSV cada vez
    else:
        df = load_from_csv("Pruebas1.csv") #cargar el archivo csv con los datos de los estudiantes usando la función de carga de datos, que devuelve un DataFrame de Polars
    
    df.write_parquet("characteristics.parquet") #guardar el DataFrame procesado en un archivo parquet para su uso posterior
    df = df.with_columns(
        AM = discretize_column(df["AM"], 5),
        RM = discretize_column(df["RM"], 5),
        CM = discretize_column(df["CM"], 5),
        BE = discretize_column(df["BE"], 5),
        EE = discretize_column(df["EE"], 5),
        CE = discretize_column(df["CE"], 5)
    ) #discretizar las columnas AM, RM, CM, BE, EE y CE en 5 bins usando la función de discretización definida anteriormente
    print(df) #imprimir el DataFrame para verificar que se haya procesado correctamente
    hypergraph_from_dataframe(df.select(pl.exclude("Id", "TND"))) #crear el hipergrafo de características a partir del dataframe
    
    
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
