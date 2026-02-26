import polars as pl
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
    VARK_scores = get_VARK_scores(students.select([f"VARK{i}" for i in range(1,14)]))
    
    #PROCESAR IM
    IM_scores = get_IM_scores_from_df(students.select(["IM1", "IM2", "IM3"]))
    
    #PROCESAR TND, MOTIVACIONES y COMPROMISO
    students = students.with_columns(
        TND = get_NDD_bitmask(students["TND"]),
        Cronotipo = (pl.col("Cronotipo") == "Entre las 7 am y las 3pm").cast(pl.UInt8), #Convertimos el cronotipo a 0 vespertino, 1 matutino
        AM = ((pl.col("AM1") + pl.col("AM2") + pl.col("AM3"))/21).round(2), #Motivación de Autonomía -> de qué tan libres se sienten los estudiantes para expresar sus ideas y opiniones, y para elegir sus actividades académicas
        RM = ((pl.col("RM1") + pl.col("RM2") + pl.col("RM3"))/21).round(2), #Motivación de Relación
        CM = ((pl.col("CM1") + pl.col("CM2"))/14).round(2), #Motivación de Competencia -> de qué tan capaces se sienten los estudiantes respecto a sus actividades académicas
        BE = ((pl.col("BE1") + pl.col("BE2") + pl.col("BE3") + pl.col("BE4")+pl.col("BE5"))/25).round(2), # Behavioural Engagement -> Compromiso Conductual
        EE = ((pl.col("EE1") + pl.col("EE2") + pl.col("EE3") + pl.col("EE4")+pl.col("EE5"))/25).round(2), # Emotional Engagement -> Compromiso Emocional 
        CE = ((pl.col("CE1") + pl.col("CE2") + pl.col("CE3") + pl.col("CE4")+pl.col("CE5"))/25).round(2) # Cognitive Engagement -> Compromiso Cognitivo
    ).select([ #Seleccionar solo las columnas relevantes para el hipergrafo
        "Id", "Cronotipo", "TND", "AM", "RM", "CM", "BE", "EE", "CE"
    ])
    
    
    #Agregar VARK al DataFrame de estudiantes
    students = students.hstack(VARK_scores) #hstack=horizontal stack -> Agrega columnas lado a lado -> agregar las columnas de VARK_scores al DataFrame de estudiantes
    
    #Agregar IM al DataFrame de estudiantes
    students = students.hstack(IM_scores) #Agrega las columnas de IM_scores al DataFrame de estudiantes
    
    return students #devuelve el DataFrame final             
            
def get_NDD_bitmask(tnd_series : pl.Series) -> pl.Series: #tnd_series es una serie de texto con los diagnósticos de NDD de cada estudiante, separados por punto y coma ->  Devuelve: Serie de enteros (UInt8) donde cada bit representa la presencia o ausencia de un trastorno
    """
    Given the string of NDD diagnostics, convert them to a bitmask

    Args:
        tnd_string (str): String containing the NDD diagnostics separated by semicolons.

    Returns:
        int: Bitmask representing the presence of NDD diagnostics
    """
    #DataFrame auxiliar (temporal) para procesar los datos de NDD y convertirlos a bitmask
    aux_df = pl.DataFrame()
    aux_df = aux_df.with_columns(
        answers = tnd_series.str.to_lowercase().str.split(';'), #Convierte el texto a minúsculas y luego lo divide en una lista de respuestas, separando por punto y coma
        TND = pl.Series("TND", [0]*len(tnd_series), dtype=pl.UInt8) #Crea una nueva columna "TND" con el mismo número de filas que tnd_series, inicializada en 0, y con tipo de dato UInt8 (entero sin signo de 8 bits)
    )
    
    #Iterar sobre cada NDD en NDD_LIST y actualizar la columna TND usando operaciones de bitwise OR para establecer el bit correspondiente si el NDD está presente en las respuestas del estudiante
    for i in range(len(NDD_LIST)):
        ndd = NDD_LIST[i] #nombre del trastorno correspondiente al bit i
        aux_df = aux_df.with_columns(
            TND = pl.when( #Si el estudiante tiene ese trastorno en su lista
                pl.col("answers").list.contains(ndd)       
            ).then( #entonces, Se enciende el bit i usando: (1 << i) -> bitmask | -> OR binario
                pl.col("TND") | (1 << i)
            ).otherwise( #Si no tiene el trastorno, el número no cambia
                pl.col("TND")
            )
        )

    return aux_df["TND"] #Se devuelve SOLO la columna TND como Serie



#Calcula el puntaje de cada tipo de inteligencia por bloque de preguntas
def get_IM_scores(im_answers: str, answer_list : list[str]) -> dict[str, int]:
    """
    Given a list of answers to the Multiple Intelligences questionnaire,
    return a dictionary with the scores for each intelligence type.
    
    Args:
        im_answers (list[str]): List of answers to the Multiple Intelligences questionnaire.
    """
    # im_answers -> string de frases ORDENADAS (un solo bloque IM)
    # answer_list: Lista fija de todas las frases posibles del bloque -> Cada posición representa una inteligencia específica
    # Devuelve un diccionario {inteligencia: puntaje}

    # Inicializamos el diccionario de resultados -> Cada inteligencia empieza con puntaje 0
    scores = {intelligence: 0 for intelligence in INTELLIGENCE_BY_INDEX}
    
    #Convertimos el string im_answers en una lista ordenada: Convertimos el string "a;b;c" → ["a", "b", "c"]
    #Convertimos cada frase a minúsculas y quitamos espacios al inicio y final para facilitar la comparación con answer_list ->la convierte en lista de frases -> list[str]
    im_answers = [ 
        phrase.strip().lower()
        for phrase in im_answers.split(";")
        if phrase.strip() != ""
    ]
    # Número total de frases (9)
    n = len(im_answers)
    
    # Recorremos las frases EN EL ORDEN QUE LAS PUSO EL ESTUDIANTE
    # enumerate nos da:
    #   position -> la posición (0 es la más importante)
    #   phrase   -> la frase en esa posición
    for position, phrase in enumerate(im_answers):

        #Convertimos la posición en un peso -> La posición 0 tiene peso 9 (n), la posición 1 tiene peso n-1, ..., la posición n-1 tiene peso 1
        weight = n - position
        phrase_index = answer_list.index(phrase) # Obtenemos el índice real de la frase
        intelligence = INTELLIGENCE_BY_INDEX[phrase_index] # Usamos ese índice para saber qué inteligencia es
        scores[intelligence] += weight # Sumamos el peso a la inteligencia correspondiente
        
    return scores # Devolvemos el resultado del bloque


def get_IM_scores_from_df(im_answers: pl.DataFrame) -> pl.DataFrame:
    """
    Dado un DataFrame con las respuestas a los bloques de Inteligencias Múltiples (IM1, IM2, IM3) devuelve un DataFrame con los puntajes de cada tipo de inteligencia sumando los 3 bloques.
    
    Args:
        im_answers (pl.DataFrame): DataFrame con las respuestas a los bloques de Inteligencias Múltiples.
        
    Returns:
        pl.DataFrame: DataFrame con los puntajes de cada tipo de inteligencia sumando los 3 bloques.
    """
    # Calcula los puntajes de cada tipo de inteligencia para cada bloque (IM1, IM2, IM3)
    """
        map_elements nos permite aplicar la función get_IM_scores a cada fila de las columnas IM1, IM2 e IM3
        lambda x: get_IM_scores(x, ANSWER_LISTS["IM1"]) es como poner:
                    def funcion_temporal(x):
                        return get_IM_scores(x, ANSWER_LISTS["IM1"])
        x representa la respuesta del estudiante a ese bloque específico (IM1, IM2 o IM3) -> se convierte en una lista de frases -> se compara con la lista de respuestas correctas para ese bloque -> se obtiene un diccionario con el puntaje de cada inteligencia para ese bloque
        """
    im_answers = im_answers.with_columns(
        IM1_scores = pl.col("IM1").map_elements(lambda x: get_IM_scores(x, ANSWER_LISTS["IM1"])),
        IM2_scores = pl.col("IM2").map_elements(lambda x: get_IM_scores(x, ANSWER_LISTS["IM2"])),
        IM3_scores = pl.col("IM3").map_elements(lambda x: get_IM_scores(x, ANSWER_LISTS["IM3"]))
    ).select(["IM1_scores", "IM2_scores", "IM3_scores"]) #Seleccionamos solo las columnas con los puntajes de cada bloque, para luego sumarlos y obtener el puntaje final de cada inteligencia
    
    # Crear un DataFrame resultado con una columna por cada tipo de inteligencia
    result = pl.DataFrame()
    
    # Para cada tipo de inteligencia, sumar los puntajes de los 3 bloques
    for intelligence in INTELLIGENCE_BY_INDEX:
        result = result.with_columns(
            pl.Series( #El nombre de la columna es el nombre de la inteligencia, y su valor es la suma de los puntajes de esa inteligencia en los 3 bloques
                intelligence,
                [
                    (((im_answers["IM1_scores"][i].get(intelligence, 0) + #se obtiene el puntaje de esa inteligencia en el bloque IM1 usando .get(intelligence, 0) para manejar el caso de que esa inteligencia no tenga puntaje en ese bloque (devuelve 0 en ese caso)
                     im_answers["IM2_scores"][i].get(intelligence, 0) +
                     im_answers["IM3_scores"][i].get(intelligence, 0))*100)/135) #se suma el puntaje de esa inteligencia en los 3 bloques, se divide por el puntaje máximo posible (135) y se multiplica por 100 para obtener un porcentaje
                    for i in range(len(im_answers)) #iteramos sobre cada estudiante (cada fila del DataFrame im_answers) para calcular el puntaje total de esa inteligencia sumando los puntajes de los 3 bloques para ese estudiante
                ]
            )
        )
        #convertimos cada resultado en un porcentaje de dos decimales
        result = result.with_columns(
            pl.col(intelligence).round(2) #Redondeamos el puntaje de esa inteligencia a 2 decimales 
        )
    
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
    result = result.with_columns( #Creamos una columna temporal llamada "ranking_dict" que es una estructura con todas las inteligencias y sus puntajes para cada estudiante, para luego aplicar map_elements y obtener un diccionario con el ranking de cada inteligencia para ese estudiante
        pl.struct(INTELLIGENCE_BY_INDEX) #toma las columnas de las inteligencias y las convierte en una estructura (similar a un diccionario) para cada fila del DataFrame
        .map_elements(#Aplica una función a cada fila de esa estructura -> la función toma como argumento la struct de inteligencias y puntajes -> Para cada estudiante ejecuta el lambda row.
            lambda row: {
                k: 1 + len( #Para cada inteligencia k, el ranking es 1 + la cantidad de inteligencias distintas que tienen un puntaje mayor que esa inteligencia k
                    {v for v in row.values() if v > row[k]} #set comprehension que crea un conjunto de los puntajes de las inteligencias que son mayores que el puntaje de la inteligencia k sin permitir repeticiones (porque si hay varias inteligencias con el mismo puntaje, todas deberían tener el mismo ranking)
                )
                for k in row
            }
        )
        .alias("ranking_dict_IM")
    )

    # Extraer cada ranking como columna
    for intelligence in INTELLIGENCE_BY_INDEX:
        result = result.with_columns(
            pl.col("ranking_dict_IM") #Toma la columna "ranking_dict_IM"
            .map_elements(lambda d: d[intelligence]) #Aplica una función a cada fila de esa columna, donde d es el diccionario con el ranking de cada inteligencia para ese estudiante, y devuelve el ranking de la inteligencia específica que queremos extraer -> crea una nueva columna con el ranking de esa inteligencia
            .alias(intelligence) #Le da a esa nueva columna el nombre de la inteligencia, para que al final tengamos una columna con el ranking de cada inteligencia para cada estudiante
        )

    # Dejar solo las columnas finales
    result = result.select(INTELLIGENCE_BY_INDEX)
        
    return result  #Devolvemos el DataFrame con los puntajes finales de cada inteligencia



def get_VARK_scores(vark_answers: pl.DataFrame) -> pl.DataFrame:
    """
    Given a list of answers to the VARK questionnaire,
    return a dictionary with the scores for each VARK type.
    
    Args:
        vark_answers (list[str]): List of answers to the VARK questionnaire.
    """
    
    vark_answers = vark_answers.with_columns(
        Answers=pl.concat_list([ #Concatena las respuestas de las columnas VARK1 a VARK13 en una sola lista de respuestas por estudiante
            pl.col(f"VARK{i}").str.to_lowercase() #Convierte las respuestas a minúsculas para facilitar la comparación con las listas de respuestas correctas (VISUAL_ANSWERS, AURAL_ANSWERS, etc.)
            .str.split(";") for i in range(1,14)]) #separa múltiples respuestas
        .list.set_difference(['']), # Elimina respuestas vacías ""
    ).select("Answers") #Selecciona solo la columna "Answers" que contiene la lista de respuestas de cada estudiante, para luego calcular el puntaje de cada tipo de aprendizaje en base a la intersección de las respuestas del estudiante con las listas de respuestas correctas para cada tipo (VISUAL_ANSWERS, AURAL_ANSWERS, etc.)
    
    vark_answers = vark_answers.with_columns(
        Visual = pl.col("Answers").list.set_intersection(VISUAL_ANSWERS).list.len()/pl.col("Answers").list.len(), #Calcula el puntaje de aprendizaje visual como la cantidad de respuestas correctas para visual (intersección entre las respuestas del estudiante y VISUAL_ANSWERS) dividido por la cantidad total de respuestas del estudiante (longitud de la lista de respuestas) -> Cuenta cuántas respuestas pertenecen al conjunto VISUAL y divide entre el total de respuestas
        Aural = pl.col("Answers").list.set_intersection(AURAL_ANSWERS).list.len()/pl.col("Answers").list.len(),
        ReadWrite = pl.col("Answers").list.set_intersection(READ_WRITE_ANSWERS).list.len()/pl.col("Answers").list.len(),
        Kinesthetic = pl.col("Answers").list.set_intersection(KINESTHETIC_ANSWERS).list.len()/pl.col("Answers").list.len(),
    )
    
    VARK_COLUMNS = ["Visual", "Aural", "ReadWrite", "Kinesthetic"] #Lista con los nombres de las columnas de VARK para luego iterar sobre ellas y crear las columnas de ranking correspondientes
    vark_answers = vark_answers.with_columns( 
        pl.struct(VARK_COLUMNS) #Creamos una estructura con las columnas de VARK para cada fila del DataFrame
        .map_elements( #Aplicamos una función a cada fila de esa estructura, donde la función toma como argumento la struct de VARK y puntajes, y devuelve un diccionario con el ranking de cada tipo de aprendizaje para ese estudiante
            lambda row: {
                k: 1 + len( #Para cada  k, el ranking es 1 + la cantidad de tipos de aprendizaje distintos que tienen un puntaje mayor que ese tipo k
                    {v for v in row.values() if v > row[k]} #set comprehension que crea un conjunto de los puntajes de los tipos de aprendizaje que son mayores que el puntaje del tipo k sin permitir repeticiones (porque si hay varios tipos con el mismo puntaje, todas deberían tener el mismo ranking
                )
                for k in row
            }
        )
        .alias("ranking_dict_VARK")          
    )
    
    # Extraer cada ranking como columna
    for vark_type in VARK_COLUMNS:
        vark_answers = vark_answers.with_columns(
            pl.col("ranking_dict_VARK") #Toma la columna "ranking_dict_VARK"
            .map_elements(lambda d: d[vark_type]) #Aplica una función a cada fila de esa columna, donde d es el diccionario con el ranking de cada tipo de aprendizaje para ese estudiante, y devuelve el ranking del tipo de aprendizaje específico que queremos extraer -> crea una nueva columna con el ranking de ese tipo de aprendizaje
            .alias(vark_type) #Le da a esa nueva columna el nombre del tipo de aprendizaje, para que al final tengamos una columna con el ranking de cada tipo de aprendizaje para cada estudiante
        )
    
    vark_answers = vark_answers.select(VARK_COLUMNS) #Seleccionamos solo las columnas finales de VARK para devolver el resultado
    
    return vark_answers