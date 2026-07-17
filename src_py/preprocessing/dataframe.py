import polars as pl
from sklearn.preprocessing import KBinsDiscretizer
from consts import *

def grade_students(students : pl.DataFrame) -> pl.DataFrame:
    """
    Given a student record, return a PyStudent object with the results of the tests
    
    Args:
        student (pl.Row): A Polars Row representing a student with at least the following fields:
            - "ID": Unique identifier for the student
            - "Cronotype": The student's cronotype
            - "TND": The student's Neurodevelopmental Disorder status regarding
                     ADHD, ADD, ASD, Dislexia, Disgraphia and Discalculia
            - "IM1": The student's answers to the first multiple intelligence set
            - "IM2": The student's answers to the second multiple intelligence set
            - "IM3": The student's answers to the third multiple intelligence set
            - "VARK 8-20": The student's answers to the VARK questionnaire
            - "Engagement 21-35": The student's answers to the engagement questionnaire
            - "Motivation 36-43": The student's answers to the motivation questionnaire
            
    Returns:
        DataFrame: A Polars DataFrame with the data obtained by grading student's formularies
    """
    #Procesar VARK
    VARK_scores = _get_VARK_scores(students.select([f"VARK{i}" for i in range(1,14)]))
    
    #PROCESAR IM
    IM_scores = _get_IM_scores_from_df(students.select(["IM1", "IM2", "IM3"]))
    
    invertedAN = {"AN2", "AN4", "AN7"}
    invertedCN = {"CN1", "CN5", "CN6"}
    invertedRN = {"RN3", "RN6", "RN7"}

    #PROCESAR TND, MOTIVACIONES y COMPROMISO
    students = students.with_columns(
        TND = _get_NDD_bitmask(students["TND"]),
        Cronotipo = (pl.col("Cronotipo") == "Entre las 7 am y las 3pm").cast(pl.UInt8) + 1, #Convertimos el cronotipo a 0 vespertino, 1 matutino
        #AN = ((pl.col("AN1") + pl.col("AN2") + pl.col("AN3"))/21).round(2), #Motivación de Autonomía -> de qué tan libres se sienten los estudiantes para expresar sus ideas y opiniones, y para elegir sus actividades académicas
        #RN = ((pl.col("RN1") + pl.col("RN2") + pl.col("RN3"))/21).round(2), #Motivación de Relación
        #CN = ((pl.col("CN1") + pl.col("CN2"))/14).round(2), #Motivación de Competencia -> de qué tan capaces se sienten los estudiantes respecto a sus actividades académicas
        AN = ((pl.sum_horizontal(*[8 - pl.col(f"AN{i}") if f"AN{i}" in invertedAN else pl.col(f"AN{i}") for i in range(1, 8)])) / 49).round(2),#Necesidad de Autonomía
        CN = ((pl.sum_horizontal(*[8 - pl.col(f"CN{i}") if f"CN{i}" in invertedCN else pl.col(f"CN{i}") for i in range(1, 7)])) / 42).round(2), #Necesidad de Competencia
        RN = ((pl.sum_horizontal(*[8 - pl.col(f"RN{i}") if f"RN{i}" in invertedRN else pl.col(f"RN{i}") for i in range(1, 9)])) / 56).round(2), #Necesidad de Relación
        BE = ((pl.col("BE1") + pl.col("BE2") + pl.col("BE3") + pl.col("BE4")+pl.col("BE5"))/25).round(2), # Behavioural Engagement -> Compromiso Conductual
        EE = ((pl.col("EE1") + pl.col("EE2") + pl.col("EE3") + pl.col("EE4")+pl.col("EE5"))/25).round(2), # Emotional Engagement -> Compromiso Emocional 
        CE = ((pl.col("CE1") + pl.col("CE2") + pl.col("CE3") + pl.col("CE4")+pl.col("CE5"))/25).round(2), # Cognitive Engagement -> Compromiso Cognitivo
    )
    
    #Dimensiones de Motivación evaluadas por el MSLQ
    students = students.with_columns(
        Orientacion_metas_intrinsecas = ((pl.col("AN") + pl.col("BE"))/2).round(2), #Orientación a metas intrínsecas
        Autoeficacia = ((pl.col("CN") + pl.col("EE"))/2).round(2), #Autoeficacia
        Valor_tarea = ((pl.col("AN") + pl.col("CN") + pl.col("BE") + pl.col("EE"))/4).round(2), #Valor de la tarea
        Ansiedad_examenes = 1 - (((pl.col("AN") + pl.col("CN") + pl.col("CE"))/3).round(2)) #Ansiedad ante exámenes
    ).select([#Seleccionar solo las columnas relevantes para el hipergrafo
        "Id", "Cronotipo", "TND", "AN", "RN", "CN", "BE", "EE", "CE", "Orientacion_metas_intrinsecas", "Autoeficacia", "Valor_tarea", "Ansiedad_examenes"])
    
    #Agregar VARK al DataFrame de estudiantes
    students = students.hstack(VARK_scores) #hstack=horizontal stack -> Agrega columnas lado a lado -> agregar las columnas de VARK_scores al DataFrame de estudiantes
    
    #Agregar IM al DataFrame de estudiantes
    students = students.hstack(IM_scores) #Agrega las columnas de IM_scores al DataFrame de estudiantes
    
    print(students)
    return students #devuelve el DataFrame final

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