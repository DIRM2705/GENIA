import polars as pl 
from xlsx2csv import Xlsx2csv #para convertir excel a csv
from grading import grade_students
from math import log10, floor
from consts import *
from group_enhancer import PyHypergraph, PyStudent

#instalé: pip install polars xlsx2csv fastexcel
#También instalé: pip install openpyxl   -> pero tengo DUDA

#Convertir excel a csv
#Xlsx2csv("C:\\Users\\Daniel\\Desktop\\Identificación de ventajas y desventajas.xlsx").convert("Pruebas1.csv")

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

df = grade_students(df)
#Imprimir esquema (schema) y datos -> diccionario interno -> Te dice: qué columnas hay y qué tipo tiene cada una
print(df.schema)

#Imprimir DataFrame
print(df)

#Obtener número de clases
n = df.height
clases = floor(1 + 3.3*log10(n))

#Clases de VARK
rango_Visual = (df['Visual'].max() - df['Visual'].min())
rango_Aural = (df['Aural'].max() - df['Aural'].min())
rango_ReadWrite = (df['ReadWrite'].max() - df['ReadWrite'].min())
rango_Kinesthetic = (df['Kinesthetic'].max() - df['Kinesthetic'].min())

ancho_clase_Visual = rango_Visual / clases
ancho_clase_Aural = rango_Aural / clases
ancho_clase_ReadWrite = rango_ReadWrite / clases
ancho_clase_Kinesthetic = rango_Kinesthetic / clases

#Clases de motivación
rango_AM = (df['AM'].max() - df['AM'].min())
rango_RM = (df['RM'].max() - df['RM'].min())
rango_CM = (df['CM'].max() - df['CM'].min())

ancho_clase_AM = rango_AM / clases
ancho_clase_RM = rango_RM / clases
ancho_clase_CM = rango_CM / clases

#Crear hipergrafo
hypergraph = PyHypergraph()
hypergraph.add_hyperedges_from_classes(clases, 'Visual')
hypergraph.add_hyperedges_from_classes(clases, 'Aural')
hypergraph.add_hyperedges_from_classes(clases, 'ReadWrite')
hypergraph.add_hyperedges_from_classes(clases, 'Kinesthetic')
hypergraph.add_hyperedges_from_classes(clases, 'AM')
hypergraph.add_hyperedges_from_classes(clases, 'RM')
hypergraph.add_hyperedges_from_classes(clases, 'CM')
for tnd in NDD_LIST:
    hypergraph.add_hyperedge(tnd)
    
    
for student in df.iter_rows(named = True):
    #Crear PyStudent
    py_student = PyStudent(
        student['Id'],
        student["TND"],
        [0]*8, #Placeholder para scores de IM
        [student['Visual'], student['Aural'], student['ReadWrite'], student['Kinesthetic']],
        0, #Placeholder para be
        0, #Placeholder para ee
        0, #Placeholder para ce
        student['AM'],
        student['RM'],
        student['CM'],
        0 #Placeholder para gpa
    )
    
    #Añadir a hipergrafo
    

hypergraph.print()
