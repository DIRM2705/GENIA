import os
from genia_libs.preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics import precision_recall_fscore_support, accuracy_score
from sklearn.svm import SVC
from pathlib import Path
from dotenv import load_dotenv
from io import StringIO
import polars as pl
import numpy as np
import joblib

def _predict_class(vocabulary : pl.DataFrame, lemas : list[str]) -> int:
    pass

def _crear_documentos(carpeta : Path) -> tuple[list[StringIO], list[str]]:
    cargar_modelo_nlp()
    
    # ==========================================================
    # Crear filas
    # ==========================================================
        
    if carpeta is None or not carpeta.exists():
        raise ValueError(f"La carpeta '{carpeta}' no existe.")

    filas = []
    clases = []

    for carpeta_clase in carpeta.iterdir():

        if not carpeta_clase.is_dir():
            continue

        clase = carpeta_clase.name

        for archivo in carpeta_clase.glob("*.pdf"):
            lemas = procesar_pdf(archivo) 
            
            doc_stream = StringIO()   
            doc_stream.write(" ".join(lemas))
            doc_stream.seek(0)  # Reset the stream position to the beginning
            filas.append(doc_stream)
            clases.append(clase)
                
    liberar_modelo_nlp()
    return (filas, clases)

if __name__ == "__main__":
    """
    load_dotenv()
    archivos, clases = _crear_documentos(Path(os.getenv("TRAINING_CURRICULUM_PATH")))
    vocabulary = TfidfVectorizer(input='file', lowercase=True, max_features=50000)
    X_train = vocabulary.fit_transform(archivos)
    print("Vocabulario creado")
    joblib.dump(vocabulary, Path("vocabulary.pkl"))

    
    model = SVC(kernel="linear", C=1.0, class_weight="balanced")
    model.fit(X_train, clases)
    
    print("Modelo entrenado")
    joblib.dump(model, "svm_model.pkl")
    """
    
    LABELS = ["Comunicacion", "Pensamiento Humano", "Pensamiento Logico", "Pensamiento Social"]
    ASSERTIONS = {
        "ARTE BUAP.pdf": LABELS[0],
        "BIOLOGIA BUAP.pdf": LABELS[2],
        "CULTURA FISICA I BUAP.pdf": LABELS[1],
        "FILOSOFIA BUAP.pdf": LABELS[1],
        "HIST UNIV MODERNA BUAP.pdf": LABELS[3],
        "INFORMATICA I BUAP.pdf": LABELS[1],
        "INFORMATICA II BUAP.pdf": LABELS[1],
        "LENGUA EXTRANJERA I BUAP.pdf": LABELS[0],
        "LENGUAJE BUAP.pdf": LABELS[0],
        "LENGUAJE E INVESTIGACION BUAP.pdf": LABELS[0],
        "MATEMATICAS I BUAP.pdf": LABELS[2],
        "MATEMATICAS II BUAP.pdf": LABELS[2],
        "PSICOLOGIA EDUCATIVA VOCACIONAL Y PROFESIOGRAFICA BUAP.pdf": LABELS[1],
        "PSICOLOGIA Y DESARROLLO HUMANO BUAP.pdf": LABELS[1],
        "QUIMICA BUAP.pdf": LABELS[2]
    }
    
    load_dotenv()
    vocabulary : TfidfVectorizer = joblib.load("vocabulary.pkl")
    
    archivos, clases = _crear_documentos(Path(os.getenv("TRAINING_CURRICULUM_PATH")))
    X_train = vocabulary.fit_transform(archivos)
    
    docs = []
    cargar_modelo_nlp()
    for pdf in Path(os.getenv("TEST_CURRICULUM_PATH")).glob("*.pdf"):
        lemas = procesar_pdf(pdf)
        doc_stream = StringIO()   
        doc_stream.write(" ".join(lemas))
        doc_stream.seek(0)
        docs.append(doc_stream)
    liberar_modelo_nlp()
    matrix = vocabulary.transform(docs)
    
    y_true = [value for _, value in ASSERTIONS.items()]
    
    metrics = {
        "c_value": [],
        "precision Comunicacion" : [],
        "precision Pensamiento Humano" : [],
        "precision Pensamiento Logico" : [],
        "precision Pensamiento Social" : [],
        "recall Comunicacion" : [],
        "recall Pensamiento Humano" : [],
        "recall Pensamiento Logico" : [],
        "recall Pensamiento Social" : [],
        "fscore Comunicacion" : [],
        "fscore Pensamiento Humano" : [],
        "fscore Pensamiento Logico" : [],
        "fscore Pensamiento Social" : [],
        "accuracy" : []
    }
    
    for c_value in range(1, 52, 10):
        model = SVC(kernel="linear", C=c_value, class_weight="balanced")
        model.fit(X_train, clases)
        result = model.predict(matrix)
        print(result)
        precision, recall, fscore, _ = precision_recall_fscore_support(y_true, result)
        accuracy = accuracy_score(y_true, result)
        metrics["c_value"].append(c_value)
        metrics["precision Comunicacion"].append(precision[0])
        metrics["precision Pensamiento Humano"].append(precision[1])
        metrics["precision Pensamiento Logico"].append(precision[2])
        metrics["precision Pensamiento Social"].append(precision[3])
        metrics["recall Comunicacion"].append(recall[0])
        metrics["recall Pensamiento Humano"].append(recall[1])
        metrics["recall Pensamiento Logico"].append(recall[2])
        metrics["recall Pensamiento Social"].append(recall[3])
        metrics["fscore Comunicacion"].append(fscore[0])
        metrics["fscore Pensamiento Humano"].append(fscore[1])
        metrics["fscore Pensamiento Logico"].append(fscore[2])
        metrics["fscore Pensamiento Social"].append(fscore[3])
        metrics["accuracy"].append(accuracy)

        
    df = pl.DataFrame(metrics)
    df.write_parquet("metrics.parquet")
    print(df.select("c_value", "^precision.*$"))
    print(df.select("c_value", "^recall.*$"))
    print(df.select("c_value", "^fscore.*$"))
    print(df.select("c_value", "accuracy"))