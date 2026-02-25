import polars as pl 
from xlsx2csv import Xlsx2csv #para convertir excel a csv
from grading import grade_students
from math import log10, floor
from consts import *
from group_enhancer import PyHypergraph, PyCharacteristicType

#instalé: pip install polars xlsx2csv fastexcel
#También instalé: pip install openpyxl   -> pero tengo DUDA

#Convertir excel a csv
#Xlsx2csv("C:\\Users\\Daniel\\Desktop\\Identificación de ventajas y desventajas.xlsx").convert("Pruebas1.csv")


pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

#Leer csv con polars, y hacer el DataFrame
df = pl.read_csv("Pruebas1.csv",infer_schema_length=1000) #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas

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
#Imprimir DataFrame
print(df.filter(pl.col("Id") == 22)) #imprimir la fila del estudiante con Id 1 para verificar que se hayan agregado las columnas de VARK y motivación correctamente

#Obtener número de clases
n = df.height #número de filas, o sea, número de estudiantes
clases = floor(1 + 3.3*log10(n)) #formula de Sturges para obtener número de clases
anchos_clases = {} #diccionario para guardar el ancho de cada clase para cada característica, para luego asignar a cada estudiante su clase correspondiente

#Clases de motivación
rango_AM = (df['AM'].max() - df['AM'].min())
rango_RM = (df['RM'].max() - df['RM'].min())
rango_CM = (df['CM'].max() - df['CM'].min())

anchos_clases['AM'] = rango_AM / clases
anchos_clases['RM'] = rango_RM / clases
anchos_clases['CM'] = rango_CM / clases

#Clases de compromiso
rango_BE = (df['BE'].max() - df['BE'].min())
rango_EE = (df['EE'].max() - df['EE'].min())
rango_CE = (df['CE'].max() - df['CE'].min())

anchos_clases['BE'] = rango_BE / clases
anchos_clases['EE'] = rango_EE / clases
anchos_clases['CE'] = rango_CE / clases

caracteristicas = { #Diccionario para mapear el nombre de la característica a su tipo en el hipergrafo
    'AM': PyCharacteristicType.AM,
    'RM': PyCharacteristicType.RM,
    'CM': PyCharacteristicType.CM,
    'BE': PyCharacteristicType.BE,
    'EE': PyCharacteristicType.EE,
    'CE': PyCharacteristicType.CE,
}

#Crear hipergrafo
hypergraph = PyHypergraph()

for item in ['AM', 'RM', 'CM', 'BE', 'EE', 'CE']:#iterar sobre cada característica para asignar a cada estudiante su clase correspondiente
    for i in range(clases):
        min_val = df[item].min() + i*anchos_clases[item] #calcular el valor mínimo de la clase i para la característica item
        max_val = df[item].min() + (i+1)*anchos_clases[item] + (i == clases-1) * 0.001 #calcular el valor máximo de la clase i para la característica item
        students  = df.select("Id", item).filter( #filtrar los estudiantes que pertenecen a la clase i para la característica item
            (pl.col(item) >= min_val) & 
            (pl.col(item) < max_val)
            )["Id"].to_list()
        hypergraph.add_students_to_characteristic(students, caracteristicas[item], i+1);#agregar los estudiantes al hipergrafo, asignándoles la clase i+1 para la característica item (i+1 porque las clases empiezan en 1 y no en 0)


#hypergraph.print()
