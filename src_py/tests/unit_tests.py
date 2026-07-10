import pytest
import os
from dotenv import load_dotenv
from pathlib import Path
from preprocessing.nlp import procesar_pdf, cargar_modelo_nlp, liberar_modelo_nlp
from main import clasificar_curriculum, cargar_modelo

#Tests del clasificador de curriculums/temarios
def clasificador_test():
    cargar_modelo_nlp()
    load_dotenv()
    path = Path(os.getenv("TEST_CURRICULUM_PATH"))
    cargar_modelo()
    for pdf in path.glob("*.pdf"):
        lemmas = procesar_pdf(pdf)
        responses = (pdf.name, clasificar_curriculum(lemmas))
        
    liberar_modelo_nlp()    