from genia_libs.preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics import precision_recall_fscore_support, accuracy_score, roc_auc_score, average_precision_score
from sklearn.svm import SVC
from pathlib import Path
from dotenv import load_dotenv
from io import StringIO
import os
import numpy as np
import joblib

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
    """"
    load_dotenv()
    archivos, clases = _crear_documentos(Path(os.getenv("TRAINING_CURRICULUM_PATH")))
    vocabulary = TfidfVectorizer(input='file', lowercase=True, max_features=50000)
    X_train = vocabulary.fit_transform(archivos)
    print("Vocabulario creado")
    joblib.dump(vocabulary, Path("experiments/curriculum_classifier/results/vocabulary.pkl"))

    
    model = SVC(kernel="linear", C=1.0, class_weight="balanced")
    model.fit(X_train, clases)
    
    print("Modelo entrenado")
    joblib.dump(model, "experiments/curriculum_classifier/results/svm_model.pkl")
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
    vocabulary : TfidfVectorizer = joblib.load("experiments/curriculum_classifier/results/vocabulary.pkl")
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
    
    
    model : SVC = joblib.load("experiments/curriculum_classifier/results/svm_model.pkl")
    result = model.predict(matrix)
    
    precision, recall, fscore, _ = precision_recall_fscore_support(y_true, result)
    accuracy = accuracy_score(y_true, result)
    
    decision_scores = model.decision_function(matrix)
    exp_matrix = np.exp(decision_scores - np.max(decision_scores, axis=1, keepdims=True))
    probabilities = exp_matrix / np.sum(exp_matrix, axis=1, keepdims=True)
    print(probabilities)
    
    roc_auc = roc_auc_score(y_true, probabilities, multi_class='ovo', average='macro')
    pr_auc = average_precision_score(y_true, probabilities, average='macro')
    
    print("Precision Comunicacion:", precision[0])
    print("Precision Pensamiento Humano:", precision[1])
    print("Precision Pensamiento Logico:", precision[2])
    print("Precision Pensamiento Social:", precision[3])
    
    print("Recall Comunicacion:", recall[0])
    print("Recall Pensamiento Humano:", recall[1])
    print("Recall Pensamiento Logico:", recall[2])
    print("Recall Pensamiento Social:", recall[3])
    
    print("F1-Score Comunicacion:", fscore[0])
    print("F1-Score Pensamiento Humano:", fscore[1])
    print("F1-Score Pensamiento Logico:", fscore[2])
    print("F1-Score Pensamiento Social:", fscore[3])
    
    print("Accuracy:", accuracy)
    print("Precision-Recall AUC Score:", pr_auc)
    print("ROC AUC Score:", roc_auc)