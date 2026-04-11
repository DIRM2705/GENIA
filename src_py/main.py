import polars as pl 
from xlsx2csv import Xlsx2csv #para convertir excel a csv
from grading import grade_students
from consts import *
from genetics import roulette_wheel
from group_enhancer import PyIndividual, GeneticAlgorithmConfig
import numpy as np

#instalé: pip install polars xlsx2csv fastexcel
#También instalé: pip install openpyxl   -> pero tengo DUDA

#Convertir excel a csv
#Xlsx2csv("C:\\Users\\Daniel\\Desktop\\Identificación de ventajas y desventajas.xlsx").convert("Pruebas1.csv")


pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

#Leer csv con polars, y hacer el DataFrame
df = pl.read_csv("Pruebas1.csv",infer_schema_length=1000) #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas

df = df.drop("Id") #eliminar la columna de Id, porque el índice de la fila ya cumple esa función, y así evitamos confusiones con la columna de Id que vamos a agregar luego
df = df.with_row_index("Id") #agregar una columna de Id para identificar a cada estudiante, con el índice de la fila como valor

#renombrar columnas para que coincidan con las esperadas en grading.py
df = df.rename({"Trabajo mejor":"Cronotipo",
           "¿Te han diagnosticado con alguno de los siguientes? Marque todas las opciones que sean válidas para usted":"TND",
           "Ordene de la más relacionada con su forma de ser a la menos relacionada con su forma de ser":"IM1", 
           "Ordene de la más relacionada con su forma de ser a la menos relacionada con su forma de ser1":"IM2", 
           "Ordene de la más relacionada con su forma de ser a la menos relacionada con su forma de ser2":"IM3", 
            "Está a punto de darle direcciones a una persona. Ella se está quedando en un hotel de su ciudad y quiere visitarlo en su casa. Ella tiene un auto rentado. Usted:":"VARK1", 
            "Usted se está quedando en un hotel y tiene un auto rentado. Le gustaría visitar a un amigo cuyo domicilio no conoce. Usted preferiría que:":"VARK2", 
            "Acaba de recibir una copia de su itinerario para un viaje por el mundo. Esto es de interés para su amigo. Usted:":"VARK3", 
            "Usted va a cocinar un postre como un regalo para su familia. Usted:":"VARK4", 
            "Usted debe instruir a un grupo de turistas sobre parques nacionales. Usted:":"VARK5", 
            "Está a punto de comprar un nuevo reproductor de música. Además del precio, ¿qué otro factor lo influenciaría a comprarlo?":"VARK6", 
            "Recuerde un momento en su vida en el que aprendió como realizar una nueva actividad como jugar un juego de mesa por primera vez. Evite elegir una habilidad física como andar en bicicleta. ¿Cómo aprend":"VARK7", 
            "¿Cuál de estos juegos prefiere?":"VARK8", 
            "Usted está a punto de aprender a usar un nuevo programa o aplicación para su computadora. Usted:":"VARK9", 
            'No está seguro si una palabra debe escribirse ""lazo"" o ""laso"". Usted:':"VARK10", 
            "Además del precio, ¿qué otro factor influenciaría su decisión de comprar un libro de texto en particular?":"VARK11", 
            "Una nueva película se acaba de estrenar. ¿Qué lo influenciaría más a ir (o no ir)?":"VARK12", 
            "Usted prefiere un profesor que use:":"VARK13", 
            "Pongo atención en el aula":"BE1", 
            "Sigo las reglas de la escuela":"BE2", 
            "Usualmente, hago mi tarea en tiempo y forma":"BE3", 
            "Cuando tengo dudas, pregunto y participo en debates dentro del aula de clases":"BE4", 
            "Usualmente participo de manera activa en trabajos grupales":"BE5", 
            "No me siento muy realizado en esta escuela":"EE1", 
            "Me emociono por el trabajo de clases":"EE2", 
            "Me gusta estar en la escuela":"EE3", 
            "Estoy interesado en realizar el trabajo escolar":"EE4", 
            "Mi aula de clases es un lugar interesante para estar":"EE5", 
            "Cuando leo un libro, me cuestiono a mi mismo para asegurarme de que estoy entendiendo el tema sobre el que estoy leyendo":"CE1", 
            "Hablo con personas fuera de la escuela sobre los temas que aprendí en clase":"CE2", 
            "Si no comprendo el significado de una palabra, trato de resolver el problema, por ejemplo consultando un diccionario o preguntándole a alguien más.":"CE3", 
            "Trato de integrar el conocimiento adquirido al resolver nuevos problemas":"CE4", 
            "Trato de integrar temas de diferentes disciplinas en mi conocimiento general":"CE5", 
            "Siento que soy libre de decidir como vivir mi vida":"AM1", 
            "Estoy cómodo con la gente con la que interactuo":"RM1", 
            "Frecuentemente, NO me siento muy competente":"CM1", 
            "Me siento presionado en mi vida":"AM2", 
            "Me llevo bien con las personas con las que estoy en contacto":"RM2", 
            "Soy mayormente reservado y no tengo muchos contactos":"RM3", 
            "Usualmente, me siento libre de expresar mis ideas y opiniones":"AM3", 
            "He sido capaz de aprender nuevas habilidades interesantes últimamente":"CM2"}) 

df = grade_students(df) #aplicamos una función que procesa las notas/puntajes de los estudiantes
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

ga = GeneticAlgorithmConfig(
    population_size=6,
    generations=10,
    mutation_rate=0.1,
    crossover_rate=0.8,
    students_data= student_array,
    students_vark_data= vark_matrix,
    students_mi_data= mi_matrix
) #crear una instancia de GeneticAlgorithmConfig con los parámetros del algoritmo genético, para facilitar


#Algoritmo genético
ind1 = PyIndividual(ga, 6)
ind2 = PyIndividual(ga, 6)
ind3 = PyIndividual(ga, 6)
ind4 = PyIndividual(ga, 6)
ind5 = PyIndividual(ga, 6)
ind6 = PyIndividual(ga, 6)

"""
population = [ind1, ind2, ind3, ind4, ind5, ind6] #población inicial de individuos
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
"""