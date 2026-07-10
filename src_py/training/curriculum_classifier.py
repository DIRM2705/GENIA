from pathlib import Path
from utils.nlp_utils import extraer_texto_pdf, procesar_texto
from collections import Counter
import polars as pl
from sklearn.feature_extraction.text import TfidfTransformer



carpeta = Path(r"../data/curriculums")


# ==========================================================
# Crear filas
# ==========================================================

filas = []

for carpeta_clase in carpeta.iterdir():

    if not carpeta_clase.is_dir():
        continue

    clase = carpeta_clase.name

    for archivo in carpeta_clase.glob("*.pdf"):

        texto = extraer_texto_pdf(archivo)

        if not texto.strip():
            continue

        lemas = procesar_texto(texto)

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

columnas = [
    c for c in df.columns 
    if c not in ("documento", "clase")
]


df_frecuencias = (
    df
    .group_by("clase")
    .agg(
        [
            pl.col(c).sum().alias(c)
            for c in columnas
        ]
    )
    .sort("clase")
)

# ==========================================================
# TF-IDF
# ==========================================================

X = df_frecuencias.drop("clase").to_numpy()

transformer = TfidfTransformer()

tfidf = transformer.fit_transform(X).toarray()


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

df_final.write_parquet(r"models/curriculm_vectors")