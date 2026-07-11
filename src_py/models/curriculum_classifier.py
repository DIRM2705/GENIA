from pathlib import Path
from preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
from collections import Counter
import polars as pl
import numpy as np
from sklearn.feature_extraction.text import TfidfTransformer
from dataclasses import dataclass

@dataclass
class CurriculumClassifier:
    
    _lemma_count : int = 0
    
    @property
    def lemma_count(self):
        return self._lemma_count
    
    def __init__(self):
        pass
    
    def predict(self, data):
        # Implement prediction logic here
        pass
    
    def train(self, carpeta : Path):
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
        )
        
        print(df_frecuencias.head(20))

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