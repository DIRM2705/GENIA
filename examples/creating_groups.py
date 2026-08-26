from genia_libs import GeneticAlgorithm
from genia_libs.utils.dataframe import load_preprocessed_lf
import matplotlib.pyplot as plt
import polars as pl

DATA_PATH = "data/test_data/Psychoeducational_Features_for_Group_Forming.parquet"
HYPERGRAPH_PATH = "data/test_data/PFGF1_hypergraph.hg"

    
if __name__ == "__main__":
    """
    Algoritmo genético para formar grupos a partir del hypergrafo.
    
    Parámetros:
    - Población inicial: 30
    - generaciones máximas: 300000
    - Spins por generación: 7
    - Elites: 2
    - Tasa de mutación: 25% (Fija)
    - Tasa de cruza: 95%
    - Número de grupos a formar: 16
    """

    #Configurar el algoritmo genético
    ga = GeneticAlgorithm(30, 300000, 7, 2, 0.95, None) 
    ga.show_config() #Mostrar la configuración del algoritmo genético
    groups = ga.run(16, HYPERGRAPH_PATH) # Formar 16 grupos a partir del hypergrafo de características
        
    for group in groups:
        df = load_preprocessed_lf(DATA_PATH).collect() #Obtiene el dataframe de características preprocesadas
        group_df = df.filter(pl.col("Id").is_in(group)) #Filtra el dataframe para obtener solo los estudiantes que pertenecen al grupo actual
        
        # Mostrar los estudiantes que pertenecen al grupo actual
        print(f"Grupo {groups.index(group) + 1}:")
        print(group_df)