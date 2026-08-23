from genia_libs._internal.consts import RECOMENDATIONS, REQUIRED_OUTPUT_COLUMNS
from genia_libs._internal.validation import validate_columns, validate_parameters
from sklearn.decomposition import PCA
import numpy as np
import polars as pl

def _needs_analysis(lf: pl.LazyFrame, n_components: int = 3):
    """
    Genera un análisis de necesidades de los estudiantes en base a sus preferencias de aprendizaje y características personales. Se utiliza PCA para reducir la dimensionalidad de los datos y determinar las variables más importantes que afectan las necesidades de los estudiantes.
    
    ## Args:
    
    - `lf (pl.LazyFrame)`: LazyFrame con los integrantes del grupo
    - `n_components (int)`: Número de variables más importantes a tomar. Defaults to 3.
        
    ## Returns:
    
    - `list[tuple[str, float]]`: Lista de tuplas con el nombre de la variable y su importancia
    """
    
    validate_columns(lf, REQUIRED_OUTPUT_COLUMNS)
    
    lf = lf.select(
        pl.exclude("VARK", "MI", "Chronotype", "AN", "RN", "CN")
    )
    
    pca = PCA(n_components = n_components)
    pca.fit(lf) #obtiene las cargas de los componentes principales
    pca.components_.T
    pesos = pca.explained_variance_ratio_ #Obtiene la varianza explicada por cada componente principal
    
    importancia = np.sum(pca.components_.T**2 * pesos, axis=1) #Se calcula la importancia de cada variable en los componentes principales
    indices=np.argsort(importancia)[::-1][:n_components] #Obtiene los indices de las variables ordenadas por importancia de las n más importantes
    
    return [(lf.collect_schema().names()[indice], importancia[indice]) for indice in indices] #Retorna una lista de tuplas con el nombre de la variable y su importancia

@validate_parameters
def make_faculty_recommendations(lf: pl.LazyFrame, n_components: int = 3) -> list[str]:
    """
    Genera recomendaciones para el profesorado en base a las necesidades de los estudiantes. Se utiliza PCA para reducir la dimensionalidad de los datos y determinar las variables más importantes que afectan las necesidades de los estudiantes.
    
    ## Args:
    
    - `lf (pl.LazyFrame)`: LazyFrame con los integrantes del grupo 
    - `n_components (int)`: Número de variables más importantes a tomar. Defaults to 3.
        
    ## Returns:
    
    - `list[str]`: Lista de recomendaciones para el profesorado
        
    ## Raises:
    
    - `ValueError`: Si no se encuentra una recomendación para alguna de las necesidades identificadas
    """
    
    needs = _needs_analysis(lf, n_components)
    
    recommendations = []
    for need in needs:
        if need[0] in RECOMENDATIONS:
            recommendations.append(RECOMENDATIONS[need[0]])
        else:
            raise ValueError(f"No se encontró una recomendación para la necesidad: {need[0]}")
    
    return recommendations