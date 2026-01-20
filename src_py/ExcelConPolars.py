import polars as pl 
from xlsx2csv import Xlsx2csv #para convertir excel a csv

#instalé: pip install polars xlsx2csv fastexcel
#También instalé: pip install openpyxl   -> pero tengo DUDA

#Convertir excel a csv
Xlsx2csv("Pruebas1 (CON MACROS).xlsm").convert("Pruebas1.csv")

#Leer csv con polars, y hacer el DataFrame
df = pl.read_csv("Pruebas1.csv",infer_schema_length=1000) #infer_schema_length para que detecte bien los tipos de datos, y analice las primeras 1000 filas

#Imprimir esquema (schema) y datos -> diccionario interno -> Te dice: qué columnas hay y qué tipo tiene cada una
print(df.schema)

#Imprimir DataFrame
print(df)

