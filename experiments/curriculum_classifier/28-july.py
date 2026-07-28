from sklearn.metrics import precision_recall_fscore_support, accuracy_score, roc_auc_score, average_precision_score
from genia_libs.models.curriculum_classifier import CurriculumClassifier
from dotenv import load_dotenv
from pathlib import Path
import joblib
import os
import numpy as np

if __name__ == "__main__":
    load_dotenv()
    
    path = Path(os.getenv("TRAINING_CURRICULUM_PATH"))
    documents = []
    classes = []
    for carpeta_clase in path.iterdir():
        if not carpeta_clase.is_dir():
            continue
        clase = carpeta_clase.name
        for pdf_file in carpeta_clase.glob("*.pdf"):
            documents.append(pdf_file)
            classes.append(clase)
            
    model = CurriculumClassifier()
    model.fit(documents, classes)
    joblib.dump(model, Path("experiments/curriculum_classifier/results/curriculum_classifier.pkl"))

    print("Modelo entrenado")
    
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
    
    test_path = Path(os.getenv("TEST_CURRICULUM_PATH"))
    documents = [file for file in test_path.glob("*.pdf") if file.is_file() and file.suffix == ".pdf"]
    
    result = model.predict(documents)   
    y_true = [value for _, value in ASSERTIONS.items()]
    
    
    precision, recall, fscore, _ = precision_recall_fscore_support(y_true, result)

    decision_scores = model.decision_function(documents)
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

    print("Precision-Recall AUC Score:", pr_auc)
    print("ROC AUC Score:", roc_auc)