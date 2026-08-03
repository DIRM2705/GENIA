from sklearn.decomposition import PCA
import numpy as np
import polars as pl

if __name__ == "__main__":
    from genia_libs.utils.dataframe import load_preprocessed_lf
    group = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    lf = load_preprocessed_lf("data/test_data/Psychoeducational_Features_for_Group_Forming.parquet")
    lf = lf.filter(pl.col("Id").is_in(group)) #Selecciona los estudiantes del grupo
    lf = lf.select(pl.exclude("MI", "VARK")) #No se consideran las preferencias de aprendizaje
    print(lf.collect().head(10)) 
    #Se preparan los datos:
    lf2 = lf.drop("Id").collect().to_numpy() #Se eliminan los Ids de los estudiantes
    nombresColumnas = lf.drop("Id").collect_schema().names() #Se obtienen los nombres de las columnas
    #PCA:
    pca = PCA(n_components = 3)
    pca.fit(lf2) #obtiene las cargas de los componentes principales
    print("Cargas de los componentes principales:")
    print(pca.components_) #Imprime las cargas de los componentes principales
    cargas = pca.components_.T  # Se transpone la matriz para que cada FILA represente una variable y cada COLUMNA una componente principal
    pesos = pca.explained_variance_ratio_ #Obtiene la varianza explicada por cada componente principal
    print("Varianza acumulada:", np.sum(pesos))
    print("Pesos (VARianza explicada) de los componentes principales:", pesos) 
    
    #Importancia de cada variable en los componentes principales (comunalidad utilizancdo la varianza o peso de cada varible):
    importancia = np.sum(cargas**2 * pesos, axis=1) #Se calcula la importancia de cada variable en los componentes principales

    indices=np.argsort(importancia)[::-1][:3] #Obtiene los indices de las variables ordenadas por importancia de las 3 más importantes
    print("Variables más importantes:")
    for indice in indices:
        print(f"{nombresColumnas[indice]}: {importancia[indice]}") #Imprime el nombre de la variable y su importancia
    """
    vectores = pca.components_.sum(axis=0) #Suma los vectores de los componentes principales
    print(f"Vector resultante: {vectores}")
    vectores_norm = vectores/np.linalg.norm(vectores) #Normaliza el vector resultante
    print(f"Vector normalizado: {vectores_norm}")
    indices = np.abs(vectores_norm).argpartition(-3)[-3:] #Obtiene los indices de los 3 valores absolutos más grandes
    print("Indices de los 3 valores absolutos más grandes:", indices)
    cols = [lf.collect_schema().names()[i] for i in indices] #Obtiene los nombres de las columnas correspondientes a los indices
    componentes =lf.select("Id", pl.col(cols)).mean().collect() #Selecciona las columnas correspondientes a los indices
    print("Componentes principales:", componentes)
    print("Varianza acumulada:", pca.explained_variance_ratio_) #Que porcentaje de información retiene cada componente
    """