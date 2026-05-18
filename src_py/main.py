import polars as pl 
from core.preprocess import load_from_csv
from core.consts import *
import numpy as np

# [DEBUG] Configuración para visualizar el DataFrame completo 
pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

df = load_from_csv("Pruebas1.csv") #cargar el archivo csv con los datos de los estudiantes usando la función de carga de datos, que devuelve un DataFrame de Polars
print(df) #imprimir el DataFrame para verificar que se haya procesado correctamente

student_array = df.select("Cronotipo", "^.*Motiv$", "^.*Engage$").to_numpy() #convertir el DataFrame a un array de numpy para facilitar su manipulación en el algoritmo genético
vark_matrix = [arr[0] for arr in df.select("VARK2").to_numpy().tolist()] #obtener la columna de VARK2 como un array de numpy
mi_matrix = [arr[0] for arr in df.select("MI1").to_numpy().tolist()] #obtener la columna de MI1 como un array de numpy
del df

#Añadir una fila al principio del array con los nombres de las columnas, para facilitar la identificación de cada columna en el algoritmo genético
header = [ 
    2, #Cronotipo
    0, #AMotiv
    0, #CMotiv
    0, #RMotiv
    0, #BEngage
    0, #EEngage
    0] #CEngage

student_array = np.vstack([header, student_array]) #agregar los nombres de las columnas como la primera fila del array

ga = GeneticAlgorithm(
    population_size=15,
    generations=10,
    mutation_rate=10,
    crossover_rate=50,
    students_data= student_array,
    students_vark_data= vark_matrix,
    students_mi_data= mi_matrix
) #crear una instancia de GeneticAlgorithm con los parámetros del algoritmo genético, para facilitar


population = ga.initialize_population(num_groups = 6) #inicializar la población de individuos (agrupamientos) usando la función de inicialización del algoritmo genético, que crea agrupamientos aleatorios de estudiantes
fit_values = [ind.get_fitness() for ind in population] #calcular el valor fitness de cada individuo en la población -> #valor fitness del individuo = qué tan bueno es el agrupmiento
print() 

selected_individual_1, idx1 = roulette_wheel(population, fit_values) #Seleccionar un individuo de la población usando la selección por ruleta -> parent (padre)
#Imprimir qué agrupamiento fue seleccionado
print("Selected individual 1: ", idx1+1)
print("Fitness of selected individual 1: ", fit_values[idx1]) 
print()
selected_individual_2, idx2 = roulette_wheel(population, fit_values)
print("Selected individual 2: ", idx2+1)
print("Fitness of selected individual 2: ", fit_values[idx2])

#Realizar el crossover entre los dos individuos seleccionados usando la función de crossover del algoritmo genético
ga.crossover(selected_individual_1, selected_individual_2) 