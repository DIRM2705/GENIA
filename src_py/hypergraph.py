import polars as pl
from group_enhancer import CharacteristicHG

def add_to_continous(df : pl.DataFrame, hg : CharacteristicHG, col_name : str, base_idx :int):
    min_val = 0
    for i in range(5):
        students = df.filter((pl.col(col_name) > min_val) & (pl.col(col_name) <= min_val + 0.2)).select("Id").to_series().to_list()
        print("Añadiendo estudiantes con {} {}: {}".format(col_name, i+1, students))
        for student in students:
            #Insertar al estudiante en el hipergrafo de la característica correspondiente,
            #en la hiperarista correspondiente a su rango de valor de la característica
            hg.add_to_hyperedge(student, base_idx + i) 
        
        min_val += 0.2
        
def add_to_discrete(df : pl.DataFrame, hg : CharacteristicHG, col_name : str, idx :int):
    students = df.filter(pl.col(col_name).is_in([1,2])).select("Id").to_series().to_list()
    print("Añadiendo estudiantes con {} {}: {}".format(col_name, 1, students))
    for student in students:
        #Insertar al estudiante en el hipergrafo de la característica correspondiente,
        #en la hiperarista correspondiente a su valor de la característica
        hg.add_to_hyperedge(student, idx)