import polars as pl
from consts import *
from group_enhancer import CharacteristicHG

def add_to_continous(df : pl.DataFrame, hg : CharacteristicHG, col_name : str, base_idx :int):
    min_val = 0
    for i in range(5):
        students = df.filter((pl.col(col_name) > min_val) & (pl.col(col_name) <= min_val + 0.2)).select("Id").to_series().to_list()
        for student in students:
            #Insertar al estudiante en el hipergrafo de la característica correspondiente,
            #en la hiperarista correspondiente a su rango de valor de la característica
            hg.add_to_hyperedge(student, base_idx + i) 
        
        min_val += 0.2
        
def add_to_discrete(df : pl.DataFrame, hg : CharacteristicHG, col_name : str, idx :int):
    students = df.filter(pl.col(col_name).is_in([1,2])).select("Id").to_series().to_list()
    for student in students:
        #Insertar al estudiante en el hipergrafo de la característica correspondiente,
        #en la hiperarista correspondiente a su valor de la característica
        hg.add_to_hyperedge(student, idx)

def create_hypergraph(df : pl.DataFrame) -> CharacteristicHG:
    hg = CharacteristicHG(df.height)
    add_to_continous(df, hg, "AMotiv", AM_BASE_IDX) #Añadir a caracteristicas de motivación autónoma
    add_to_continous(df, hg, "RMotiv", RM_BASE_IDX) #Añadir a caracteristicas de motivación de relación
    add_to_continous(df, hg, "CMotiv", CM_BASE_IDX) #Añadir a caracteristicas de motivación de competencia
    add_to_continous(df, hg, "BEngage", BE_BASE_IDX) #Añadir a caracteristicas de compromiso conductual
    add_to_continous(df, hg, "EEngage", EE_BASE_IDX) #Añadir a caracteristicas de compromiso emocional
    add_to_continous(df, hg, "CEngage", CE_BASE_IDX) #Añadir a caracteristicas de compromiso cognitivo

    for i in range(len(VARK_COLUMNS)):
        add_to_discrete(df, hg, VARK_COLUMNS[i], VARK_BASE_IDX + i) #Añadir a caracteristicas de estilo de aprendizaje 
        
    for i in range(len(INTELLIGENCE_BY_INDEX)):
        add_to_discrete(df, hg, INTELLIGENCE_BY_INDEX[i], INTELLIGENCE_BASE_IDX + i) #Añadir a caracteristicas de inteligencia