import polars as pl
from consts import REQUIRED_INPUT_COLUMNS, MI_COLUMNS, VARK_COLUMNS

def preprocess(students : pl.LazyFrame) -> pl.DataFrame:
    """
    Dado un archivo en formato csv, crea un dataframe de polars creando las columnas necesarias
    
    Args:
        file_path (str): La ruta de un archivo csv con las siguientes columnas:
            - "Id": Identificador único del estudiante
            - "Cronotype": El cronotipo del estudiante
            - "AN": Porcentaje de satisfacción de la necesidad de autonomía
            - "RN": Porcentaje de satisfacción de la necesidad de relaciones
            - "CN": Porcentaje de satisfacción de la necesidad de competencia
            - "BE": Porcentaje de compromiso conductual
            - "EE": Porcentaje de compromiso emocional
            - "CE": Porcentaje de compromiso cognitivo
            - "HS": Porcentaje de búsqueda de ayuda
            - "PL": Porcentaje de aprendizaje por pares
            - "TM": Porcentaje de manejo del tiempo
            - "RH": Porcentaje de repetición
            - "EL": Porcentaje de elaboración
            - "OR": Porcentaje de organización
            - "CP": Porcentaje de pensamiento crítico
            - "MC": Porcentaje de metacognición
            - "MIKin": Puntaje de la inteligencia múltiple cinestésica
            - "MIExis": Puntaje de la inteligencia múltiple existencial
            - "MIInter": Puntaje de la inteligencia múltiple interpersonal
            - "MIIntra": Puntaje de la inteligencia múltiple intrapersonal
            - "MILog": Puntaje de la inteligencia múltiple lógico-matemática
            - "MIMus": Puntaje de la inteligencia múltiple musical
            - "MINat": Puntaje de la inteligencia múltiple naturalista
            - "MIVer": Puntaje de la inteligencia múltiple verbal
            - "MIVis": Puntaje de la inteligencia múltiple visual
            - "VARKVisual": Puntaje del estilo de aprendizaje visual
            - "VARKAural": Puntaje del estilo de aprendizaje auditivo
            - "VARKReadWrite": Puntaje del estilo de aprendizaje lectura/escritura
            - "VARKKinesthetic": Puntaje del estilo de aprendizaje kinestésico
            
    Returns:
        DataFrame: Un dataframe de polars con el formato requerido
    """
    
    _verify_columns(students) #Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    #Procesar VARK
    students = _grade_VARK_scores(students) #Selecciona las columnas de VARK y las procesa para obtener los puntajes de VARK
    
    #Procesar IM
    students = _grade_IM_scores(students) #Selecciona las columnas de IM y las procesa para obtener los puntajes de IM

    #Procesar motivaciones
    students = _grade_motivations(students)
    
    return students.collect() #devuelve el DataFrame final   

def _verify_columns(df: pl.DataFrame):
    """
    Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    Args:
        df (pl.DataFrame): DataFrame a verificar
        
    Raises:
        ValueError: Si falta alguna columna necesaria
    """
    
    missing_columns = [col for col in REQUIRED_INPUT_COLUMNS if col not in df.collect_schema().names()]
    
    if missing_columns:
        raise ValueError(f"Las siguientes columnas no existen y son necesarias en el DataFrame: {missing_columns}")

def _grade_motivations(students : pl.LazyFrame) -> pl.LazyFrame:
    """
    Dado el porcentaje de satisfacción de cada necesidad genera un dataframe 
    con las dimensiones de motivación del MSLQ
    
    Args:
        student (pl.LazyFrame): Un LazyFrame de polars con al menos las siguientes columnas:
            - "Id": Identificador único del estudiante
            - "AN": Porcentaje de satisfacción de la necesidad de autonomía
            - "RN": Porcentaje de satisfacción de la necesidad de relaciones
            - "CN": Porcentaje de satisfacción de la necesidad de competencia
            - "BE": Porcentaje de compromiso conductual
            - "EE": Porcentaje de compromiso emocional
            - "CE": Porcentaje de compromiso cognitivo
            
    Returns:
        LazyFrame: El LazyFrame con las dimensiones de motivación del MSLQ añadidas como nuevas columnas
    """
    
    #Dimensiones de Motivación evaluadas por el MSLQ
    motivations = students.with_columns(
        EGO = ((pl.col("CN") + pl.col("BE"))/2).round(4), #Orientación a metas extrínsecas
        IGO = ((pl.col("AN") + pl.col("BE"))/2).round(4), #Orientación a metas intrínsecas
        SE = ((pl.col("CN") + pl.col("EE"))/2).round(4), #Autoeficacia
        TV = ((pl.col("AN") + pl.col("CN") + pl.col("BE") + pl.col("EE"))/4).round(4), #Valor de la tarea
        TA = 1 - (((pl.col("AN") + pl.col("CN") + pl.col("CE"))/3).round(4)) #Ansiedad ante exámenes
    )
    
    return motivations #devuelve el LazyFrame con las nuevas columnas de motivación añadidas         

def _grade_IM_scores(mi_df: pl.LazyFrame) -> pl.LazyFrame:
    """
    Dado un LazyFrame con el puntaje asignado a cada inteligencia múltiple genera un dataframe
    que rankea cada inteligencia
    
    Args:
        students (pl.LazyFrame): LazyFrame con el puntaje de cada inteligencia múltiple,
        con al menos las siguientes columnas:
            - "Id": Identificador único del estudiante
            - "MIKin": Puntaje de la inteligencia cinestésica
            - "MIExis": Puntaje de la inteligencia existencial
            - "MIInter": Puntaje de la inteligencia interpersonal
            - "MIIntra": Puntaje de la inteligencia intrapersonal
            - "MILog": Puntaje de la inteligencia lógico-matemática
            - "MIMus": Puntaje de la inteligencia musical
            - "MINat": Puntaje de la inteligencia naturalista
            - "MIVer": Puntaje de la inteligencia verbal
            - "MIVis": Puntaje de la inteligencia visual
        
    Returns:
        pl.LazyFrame: LazyFrame que le asigna una posición a cada inteligencia
    """
    
    # Crear RANKING por estudiante (1 = mayor puntaje)
    # Creamos una columna temporal llamada "ranking_dict" que es una estructura con todas las inteligencias y
    # sus puntajes para cada estudiante, para luego aplicar map_elements y
    # obtener un diccionario con el ranking de cada inteligencia para ese estudiante
    result = mi_df.with_columns( 
        pl.struct(MI_COLUMNS)
        # Para cada inteligencia k,
        # el ranking es 8 menos la cantidad de inteligencias distintas que tienen un puntaje menor que esa inteligencia k
        .map_elements(
            lambda row: {
                k: (len( 
                    {v for v in row.values() if v < row[k]}
                ))
                for k in row
            }
        )
        .alias("ranking_dict_IM")
    )
    
    #Toma la columna "ranking_dict_IM" que contiene el diccionario con el ranking de las IM
    result = result.with_columns( 
        MI = pl.col("ranking_dict_IM") 
        .map_elements(
            lambda element:
                [
                    # Ranking es 0 significa el puntaje más alto para ese estudiante
                    MI_COLUMNS.index(k) for k, v in element.items() if v == 0 
                ],
                pl.List(pl.UInt8)
        )
        .cast(pl.List(pl.UInt8))
    )
    
    # Eliminar las columnas originales
    result = result.select(pl.exclude(MI_COLUMNS + ["ranking_dict_IM"])) #Seleccionamos solo las columnas finales de MI para devolver el resultado
        
    return result  #Devolvemos el DataFrame con los puntajes finales de cada inteligencia

def _grade_VARK_scores(vark_df: pl.LazyFrame) -> pl.LazyFrame:
    """
    Dado un lazyframe con los puntajes asignados a cada estilo de aprendizaje añade al lazyframe
    la columna de rankeo
    
    Args:
        vark_df (pl.LazyFrame): Lazyframe con el puntaje de los estilos de aprendizaje,
        con al menos las siguientes columnas:
            - "Id": Identificador único del estudiante
            - "VARKVisual": Puntaje del estilo de aprendizaje visual
            - "VARKAural": Puntaje del estilo de aprendizaje auditivo
            - "VARKReadWrite": Puntaje del estilo de aprendizaje lectura/escritura
            - "VARKKinesthetic": Puntaje del estilo de aprendizaje kinestésico
        
    Returns:
        pl.LazyFrame: Lazyframe con la columna de rankeo añadida
    """
    
    vark_df = vark_df.with_columns( 
        pl.struct(VARK_COLUMNS) #Creamos una estructura con las columnas de VARK para cada fila del DataFrame
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
                    VARK_COLUMNS.index(k) for k, v in element.items() if v == 0 #Si el ranking es 0, significa que ese tipo de aprendizaje tiene el puntaje más alto para ese estudiante, por lo que se asigna un valor de 1 a esa variable y 0 a las demás
                ],
                pl.List(pl.UInt8)
        )
        .cast(pl.List(pl.UInt8)), #Convertimos la lista de rankings a una lista de UInt8 para facilitar su manipulación en el algoritmo genético
    )
    
    vark_df = vark_df.select(pl.exclude(VARK_COLUMNS + ["ranking_dict_VARK"])) #Seleccionamos solo las columnas finales de VARK para devolver el resultado
    
    return vark_df