import polars as pl
from sklearn.preprocessing import KBinsDiscretizer
from consts import *

def discretize_column(column: pl.Series, n_bins: int) -> pl.Series:
    """
    Discretize a continuous column into n_bins using KBinsDiscretizer from sklearn.

    Args:
        column (pl.Series): The continuous column to be discretized.
        n_bins (int): The number of bins to discretize into.

    Returns:
        pl.Series: A new Polars Series with the discretized values.
    """
    # Convertir la columna de Polars a un array de numpy para usar con KBinsDiscretizer
    column_np = column.to_numpy().reshape(-1, 1) #reshape para convertirlo en una matriz de una sola columna, que es lo que espera KBinsDiscretizer
    
    # Crear el discretizador con n_bins y estrategia de cuantiles para que cada bin tenga aproximadamente la misma cantidad de muestras
    discretizer = KBinsDiscretizer(n_bins=n_bins, encode='ordinal', strategy='kmeans')
    
    # Ajustar el discretizador a los datos y transformarlos
    discretized_np = discretizer.fit_transform(column_np).astype(int).flatten() #fit_transform para ajustar el modelo y transformar los datos, astype(int) para convertir los valores a enteros, flatten() para convertir la matriz resultante en un array unidimensional
    
    # Convertir el array de numpy resultante de nuevo a una Serie de Polars
    discretized_series = pl.Series(discretized_np, dtype=pl.UInt8) #Convertimos a UInt8 para ahorrar espacio, ya que el número de bins es pequeño
    
    return discretized_series          

def grade_IM_scores(mi_df: pl.DataFrame) -> pl.DataFrame:
    """
    Dado un DataFrame con el puntaje asignado a cada inteligencia múltiple genera un dataframe
    que rankea cada inteligencia
    
    Args:
        im_answers (pl.DataFrame): DataFrame con las respuestas a los bloques de Inteligencias Múltiples.
        
    Returns:
        pl.DataFrame: DataFrame que le asigna una posición a cada inteligencia
    """

    # Crear un DataFrame resultado con una columna por cada tipo de inteligencia
    mi_df = pl.DataFrame()
    
    # Crear RANKING por estudiante (1 = mayor puntaje)
    """
            Si tengo Visual 16.9, Musical: 14.5, Logica: 20.2
            Se crea una estructura asi con el pl.Struct:
            {
                "Visual": 16.9, 
                "Musical": 14.5, 
                "Logica": 20.2
            }
    """
    result = mi_df.with_columns( #Creamos una columna temporal llamada "ranking_dict" que es una estructura con todas las inteligencias y sus puntajes para cada estudiante, para luego aplicar map_elements y obtener un diccionario con el ranking de cada inteligencia para ese estudiante
        pl.struct(INTELLIGENCE_BY_INDEX) #toma las columnas de las inteligencias y las convierte en una estructura (similar a un diccionario) para cada fila del DataFrame
        .map_elements(#Aplica una función a cada fila de esa estructura -> la función toma como argumento la struct de inteligencias y puntajes -> Para cada estudiante ejecuta el lambda row.
            lambda row: {
                k: (len( #Para cada inteligencia k, el ranking es 8 menos la cantidad de inteligencias distintas que tienen un puntaje menor que esa inteligencia k
                    {v for v in row.values() if v < row[k]} #set comprehension que crea un conjunto de los puntajes de las inteligencias que son mayores que el puntaje de la inteligencia k sin permitir repeticiones (porque si hay varias inteligencias con el mismo puntaje, todas deberían tener el mismo ranking)
                ))#Se normaliza para ser una variable ordinal entre 0 y 1
                for k in row
            }
        )
        .alias("ranking_dict_IM")
    )
    
    result = result.with_columns( #Creamos una columna para cada tipo de inteligencia con su ranking correspondiente, usando map_elements para extraer el ranking de cada inteligencia del diccionario "ranking_dict_IM"
        MI = pl.col("ranking_dict_IM") #Toma la columna "ranking_dict_IM" que contiene el diccionario con el ranking de cada inteligencia para cada estudiante
        .map_elements(
            lambda element:
                [
                    INTELLIGENCE_BY_INDEX.index(k) for k, v in element.items() if v == 0 #Si el ranking es 0, significa que esa inteligencia tiene el puntaje más alto para ese estudiante, por lo que se asigna un valor de 1 a esa variable y 0 a las demás
                ],
                pl.List(pl.UInt8)
        )
        .cast(pl.List(pl.UInt8)), #Convertimos la lista de rankings a una lista de UInt8 para facilitar su manipulación en el algoritmo genético
    )
    
    # Dejar solo las columnas finales
    result = result.select("MI") #Seleccionamos solo las columnas finales de MI para devolver el resultado
        
    return result  #Devolvemos el DataFrame con los puntajes finales de cada inteligencia

def grade_VARK_scores(vark_df: pl.DataFrame) -> pl.DataFrame:
    """
    Dado un dataframe con los puntajes asignados a cada estilo de aprendizaje genera un dataframe
    que rankea los estilos de aprendizaje
    
    Args:
        vark_df (pl.DataFrame): Dataframe con los puntajes de cada estilo de aprendizaje
    """
    
    vark_df = vark_df.with_columns( 
        pl.struct(VARK_BY_INDEX) #Creamos una estructura con las columnas de VARK para cada fila del DataFrame
        .map_elements( #Aplicamos una función a cada fila de esa estructura, donde la función toma como argumento la struct de VARK y puntajes, y devuelve un diccionario con el ranking de cada tipo de aprendizaje para ese estudiante
            lambda row: {
                k: len( #Para cada  k, el ranking es 1 + la cantidad de tipos de aprendizaje distintos que tienen un puntaje mayor que ese tipo k
                    {v for v in row.values() if v > row[k]} #set comprehension que crea un conjunto de los puntajes de los tipos de aprendizaje que son mayores que el puntaje del tipo k sin permitir repeticiones (porque si hay varios tipos con el mismo puntaje, todas deberían tener el mismo ranking
                )
                for k in row
            }
        )
        .alias("ranking_dict_VARK")          
    )
    
    vark_df = vark_df.with_columns(
        VARK = pl.col("ranking_dict_VARK") #Toma la columna "ranking_dict_VARK"
        .map_elements(
            lambda element:
                [
                    VARK_BY_INDEX.index(k) for k, v in element.items() if v == 0 #Si el ranking es 0, significa que ese tipo de aprendizaje tiene el puntaje más alto para ese estudiante, por lo que se asigna un valor de 1 a esa variable y 0 a las demás
                ],
                pl.List(pl.UInt8)
        )
        .cast(pl.List(pl.UInt8)), #Convertimos la lista de rankings a una lista de UInt8 para facilitar su manipulación en el algoritmo genético
    )
    
    vark_answers = vark_answers.select("VARK") #Seleccionamos solo las columnas finales de VARK para devolver el resultado
    
    return vark_answers