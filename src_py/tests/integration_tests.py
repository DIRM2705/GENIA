import os
from dotenv import load_dotenv
from pathlib import Path
from preprocessing.nlp import procesar_pdf, cargar_modelo_nlp, liberar_modelo_nlp
from main import clasificar_curriculum, cargar_modelo

#Tests del clasificador de curriculums/temarios
def test_clasificador():
    cargar_modelo_nlp()
    load_dotenv()
    path = Path(os.getenv("TEST_CURRICULUM_PATH"))
    cargar_modelo()
    responses = []
    for pdf in path.glob("*.pdf"):
        lemmas = procesar_pdf(pdf)
        responses.append((pdf.name, clasificar_curriculum(lemmas)))
        
    liberar_modelo_nlp() 
        
    ASSERTIONS = {
        "ARTE BUAP.pdf": 1,
        "BIOLOGIA BUAP.pdf": 2,
        "CULTURA FISICA I BUAP.pdf": 3,
        "FILOSOFIA BUAP.pdf": 1,
        "HIST UNIV MODERNA BUAP.pdf": 3,
        "INFORMATICA I BUAP.pdf": 2,
        "INFORMATICA II BUAP.pdf": 2,
        "LENGUA EXTRANJERA I BUAP.pdf": 0,
        "LENGUAJE BUAP.pdf": 0,
        "LENGUAJE E INVESTIGACION BUAP.pdf": 0,
        "MATEMATICAS I BUAP.pdf": 2,
        "MATEMATICAS II BUAP.pdf": 2,
        "PSICOLOGIA EDUCATIVA VOCACIONAL Y PROFESIOGRAFICA BUAP.pdf": 1,
        "PSICOLOGIA Y DESARROLLO HUMANO BUAP.pdf": 1,
        "QUIMICA BUAP.pdf": 2
    }   
    
    print("Resultados de la clasificación:")

    error = 0
    for pdf_name, response in responses:
        error += 1 if response != ASSERTIONS[pdf_name] else 0
        print(f"Archivo: {pdf_name}, Clase esperada: {ASSERTIONS[pdf_name]}, Clase obtenida: {response}")
    
    print(f"Errores totales: {error}/{len(responses)}")
    
    assert error/len(responses) < 0.15, "El porcentaje de errores es mayor al 15%"
    