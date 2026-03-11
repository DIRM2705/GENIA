import polars as pl 
from xlsx2csv import Xlsx2csv #para convertir excel a csv
from grading import grade_students
from consts import *
from hypergraph import create_hypergraph
from group_enhancer import PyIndividual
import numpy as np
import random

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

print(df)

#Añadir alumnos a los hipergrafos
hg = create_hypergraph(df)

#Algoritmo genético
group1 = [[0,1,2,3,4], [5,6,7,8,9], [10,11,12,13,14,15], [16,17,18,19,20], [21,22,23,24,25], [26,27,28,29]]
group2 = [[0,6,12,18,28], [1,7,13,19,29], [2,8,14,20,24,27], [3,9,15,21,25], [4,10,16,22,26], [5,11,17,23]]
group3 = [[0,7,14,21,28], [1,8,15,22,29], [2,9,16,23,24,27], [3,10,17,24,25], [4,11,18,25,26], [5,12,19,26]]
group4 = [[0,8,16,24,28], [1,9,17,25,29], [2,10,18,26,27], [3,11,19,27,25], [4,12,20,22,26], [5,13,21,23]]
group5 = [[0,9,18,27,28], [1,10,19,26,29], [2,11,20,27,24,25], [3,12,21,28,25], [4,13,22,23,26], [5,14,23,24]]
group6 = [[0,10,24,27,28], [1,11,25,26,29], [2,12,21,22,24,25], [3,13,23,24,25], [4,14,20,21,26], [5,15,22,23,26]]
ind1 = PyIndividual(group1, df) #un individuo es una posible forma de agrupar a todos los estudiantes -> 6 grupos
ind2 = PyIndividual(group2, df)
ind3 = PyIndividual(group3, df)
ind4 = PyIndividual(group4, df)
ind5 = PyIndividual(group5, df)
ind6 = PyIndividual(group6, df)

population = [ind1, ind2, ind3, ind4, ind5, ind6] #población inicial de individuos
fit_values = [ind.fit() for ind in population] #calcular el valor fitness de cada individuo en la población -> #valor fitness del individuo = qué tan bueno es el agrupmiento
print() 

def roulette_wheel(population, fit_values):
    
    total_fitness = sum(fit_values) #Sumar los valores fitness para calcular las probabilidades de selección
    probabilities = [fit / total_fitness for fit in fit_values] #Calcular las probabilidades de selección para cada individuo
    print("Selection probabilities: ", probabilities)
    
    cumulative_probabilities = np.cumsum(probabilities) #Calcular las probabilidades acumuladas para la selección por ruleta
    print("Cumulative probabilities: ", cumulative_probabilities)
    
    r = random.uniform(0, 1) #Generar un número aleatorio uniforme entre 0 y 1 para seleccionar un individuo
    for i, cumulative_probability in enumerate(cumulative_probabilities):
        if r < cumulative_probability: #Seleccionar el primer individuo cuya probabilidad acumulada sea mayor que el número aleatorio generado
            return population[i], i #Seleccionar el individuo correspondiente a la probabilidad acumulada
        
selected_individual_1, idx1 = roulette_wheel(population, fit_values) #Seleccionar un individuo de la población usando la selección por ruleta -> parent (padre)
#Imprimir qué agrupamiento fue seleccionado
print("Selected individual 1: ", idx1+1)
print("Fitness of selected individual 1: ", fit_values[idx1]) 
print()
selected_individual_2, idx2 = roulette_wheel(population, fit_values)
print("Selected individual 2: ", idx2+1)
print("Fitness of selected individual 2: ", fit_values[idx2])