from pathlib import Path
from preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
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
    
    def fit(self, carpeta : Path):
        pass