import sys
import os

# Get the absolute path to the parent directory
parent_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '../..'))
sys.path.append(parent_dir)

from preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
from sklearn.feature_extraction.text import TfidfTransformer
from sklearn.metrics import classification_report, confusion_matrix
from pathlib import Path
from dotenv import load_dotenv
from collections import Counter
import polars as pl
import numpy as np

def _predict_class(model : pl.DataFrame, lemas : list[str]) -> int:
    vector =(model.filter(pl.col("lema").is_in(lemas)) #Filtra solo los lemas que están en el modelo
    .select(pl.all().exclude("lema")) #Selecciona vectores de características (sin la columna de lemas)
    .sum() #Suma los vectores de características
    .to_numpy()) #Convierte a numpy array
    
    print(f"Vector de características: {vector}")
    return np.argmax(vector)

def _calcular_frecuencias(carpeta : Path):
    cargar_modelo_nlp()
    
    # ==========================================================
    # Crear filas
    # ==========================================================
        
    if carpeta is None or not carpeta.exists():
        raise ValueError(f"La carpeta '{carpeta}' no existe.")

    filas = []

    for carpeta_clase in carpeta.iterdir():

        if not carpeta_clase.is_dir():
            continue

        clase = carpeta_clase.name

        for archivo in carpeta_clase.glob("*.pdf"):
            lemas = procesar_pdf(archivo)

            conteo = Counter(lemas)

            # Evitar conflictos con nombres reservados
            conteo.pop("clase", None)
            conteo.pop("documento", None)

            fila = {
                "__documento__": archivo.stem,
                "__clase__": clase
            }

            fila.update(conteo)
                

            filas.append(fila)
                
    liberar_modelo_nlp()
    return filas


def curriculum_classifier_tf_idf_per_class():
    carpeta = Path(os.getenv("TRAINING_CURRICULUM_PATH"))
    filas = _calcular_frecuencias(carpeta)
    
    # ==========================================================
    # DataFrame
    # ==========================================================

    df = (
        pl.DataFrame(filas)
            .rename({
                "__documento__": "documento",
                "__clase__": "clase"
            })
            .fill_null(0)
        )

    # ==========================================================
    # Agrupar por clase
    # ==========================================================

    df_frecuencias = (
        df
        .group_by("clase")
        .agg(
            [
                pl.all().exclude("clase", "documento").sum()
            ]     
        )
        .sort("clase")
    )
        

    # ==========================================================
    # TF-IDF
    # ==========================================================

    X = df_frecuencias.drop("clase").to_numpy()

    transformer = TfidfTransformer(smooth_idf=False)

    tfidf = transformer.fit_transform(X).toarray() / np.log10(4)


    df_tfidf = pl.DataFrame(
        tfidf,
        schema=df_frecuencias.drop("clase").columns
    )


    df_tfidf = df_tfidf.insert_column(
        0,
        pl.Series(
            "clase",
            df_frecuencias["clase"]
        )
    )

    # ==========================================================
    # Matriz transpuesta
    # ==========================================================

    df_final = pl.DataFrame(
        tfidf.T,
        schema=df_frecuencias["clase"].to_list()
    )


    df_final = df_final.insert_column(
        0,
        pl.Series(
            "lema",
            df_frecuencias.drop("clase").columns
        )
    )
        
    df_final = df_final.filter((pl.col("Comunicacion").eq(0)) | pl.col("Pensamiento Humano").eq(0) | pl.col("Pensamiento Logico").eq(0) | pl.col("Pensamiento Social").eq(0))
        
    return df_final

if __name__ == "__main__":
    load_dotenv()
    model = curriculum_classifier_tf_idf_per_class()
    ASSERTIONS = {
        "ARTE BUAP.pdf": 0,
        "BIOLOGIA BUAP.pdf": 2,
        "CULTURA FISICA I BUAP.pdf": 1,
        "FILOSOFIA BUAP.pdf": 1,
        "HIST UNIV MODERNA BUAP.pdf": 3,
        "INFORMATICA I BUAP.pdf": 1,
        "INFORMATICA II BUAP.pdf": 1,
        "LENGUA EXTRANJERA I BUAP.pdf": 0,
        "LENGUAJE BUAP.pdf": 0,
        "LENGUAJE E INVESTIGACION BUAP.pdf": 0,
        "MATEMATICAS I BUAP.pdf": 2,
        "MATEMATICAS II BUAP.pdf": 2,
        "PSICOLOGIA EDUCATIVA VOCACIONAL Y PROFESIOGRAFICA BUAP.pdf": 1,
        "PSICOLOGIA Y DESARROLLO HUMANO BUAP.pdf": 1,
        "QUIMICA BUAP.pdf": 2
    }
    
    LABELS = ["Comunicación", "Pensamiento Humano", "Pensamiento Logico", "Pensamiento Social"]
    
    y_true = [value for _, value in ASSERTIONS.items()]
    y_pred = []
    
    cargar_modelo_nlp()
    for pdf in Path(os.getenv("TEST_CURRICULUM_PATH")).glob("*.pdf"):
        lemas = procesar_pdf(pdf)
        y_pred.append(_predict_class(model, lemas))
    liberar_modelo_nlp()
    
    result = classification_report(y_true, y_pred, target_names=LABELS)
    print("Reporte de clasificación:")
    print(result)
    print("Matriz de confusión:")
    print(confusion_matrix(y_true, y_pred))
