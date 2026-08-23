from pathlib import Path
import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.svm import SVC
from io import StringIO
from genia_libs.preprocessing.nlp import cargar_modelo_nlp, liberar_modelo_nlp, procesar_pdf
from genia_libs._internal.validation import validate_parameters

class CurriculumClassifier:
    """
    Clasifica los temarios en áreas de conocimiento según el modelo de la Nueva Escuela Mexcana (NEM)
    """
    _model : SVC
    _vocabulary : TfidfVectorizer

    def __init__(self):
        self._model = SVC(kernel="linear", C=1.0, class_weight="balanced")
        self._vocabulary = TfidfVectorizer(input='file', lowercase=True, max_features=50000)

    @validate_parameters
    def fit(self, documents : list[Path | str], training_labels : list[str]) -> 'CurriculumClassifier':
        """
        Entrena el modelo de clasificación con los documentos y etiquetas de entrenamiento proporcionados.

        ## Args:
        
        - `documents (list[Path | str])`: Lista de rutas a los documentos PDF o strings que representan los documentos a utilizar para entrenar el modelo.
        - `training_labels (list[str])`: Lista de etiquetas de entrenamiento correspondientes a cada documento. Cada etiqueta debe ser una cadena que represente el área de conocimiento según el modelo de la Nueva Escuela Mexicana (NEM).

        ## Raises:
        
        - `ValueError`: Si no se proporcionan documentos o si la cantidad de documentos y etiquetas de entrenamiento no coincide.
        - `ValueError`: Si algún documento no es un archivo PDF válido.

        ## Returns:
        
        - `CurriculumClassifier`: La instancia del clasificador entrenada.
        """
        if documents is None or len(documents) == 0:
            raise ValueError("No se proporcionaron documentos para entrenar el modelo.")
        elif len(documents) != len(training_labels):
            raise ValueError("La cantidad de documentos y etiquetas de entrenamiento no coincide.")
        
        documents = [Path(doc) if isinstance(doc, str) else doc for doc in documents]
        cargar_modelo_nlp()
        
        docs = []
        
        for doc in documents:
            docs.append(self._process_document(doc))
        
        liberar_modelo_nlp()
        
        doc_matrix = self._vocabulary.fit_transform(docs)
        self._model.fit(doc_matrix, training_labels)
        
        return self
        
    def _process_document(self, pdf_file: Path) -> StringIO:
        lemas = procesar_pdf(pdf_file)
        doc_stream = StringIO()
        doc_stream.write(" ".join(lemas))
        doc_stream.seek(0)

        return doc_stream

    @validate_parameters
    def predict(self, documents : list[Path | str]) -> np.ndarray:
        """
        Realiza la predicción de las etiquetas de los documentos proporcionados.

        ## Parameters:
        
        - `documents (list[Path | str])`: Lista de rutas a los documentos PDF o strings que representan los documentos a utilizar para predecir.

        ## Returns:
        
        - `np.ndarray`: Array con las etiquetas predichas para cada documento.

        ## Raises:
        
        - `ValueError`: Si no se proporcionan documentos o si algún documento no es un archivo PDF válido.
        """
        if not documents:
            raise ValueError("No se proporcionaron documentos para predecir.")
                
        documents = [Path(doc) if isinstance(doc, str) else doc for doc in documents]

        cargar_modelo_nlp()      
        clean_docs = []
        for file in documents:
            if not file.exists() or not file.is_file() or file.suffix != ".pdf":
                raise ValueError(f"El archivo '{file}' no es un PDF válido.")
            doc = self._process_document(file)
            clean_docs.append(doc)
        liberar_modelo_nlp()
        
        doc_matrix = self._vocabulary.transform(clean_docs)
        return self._model.predict(doc_matrix)
    
    @validate_parameters
    def decision_function(self, documents: list[Path | str]) -> np.ndarray:
        """
        Devuelve la función de decisión utilizada para clasificar

        ## Args:
        - `documents (list[Path  |  str])`: Lista de rutas a los documentos PDF o strings que representan los documentos a utilizar para calcular la función de decisión.

        ## Raises:
        
        - `ValueError`: Si no se proporcionan documentos o si algún documento no es un archivo PDF válido.

        ## Returns:
        - `np.ndarray`: Array con los valores de la función de decisión para cada documento.
        """
        if not documents:
            raise ValueError("No se proporcionaron documentos para predecir.")
                
        documents = [Path(doc) if isinstance(doc, str) else doc for doc in documents]

        cargar_modelo_nlp()      
        clean_docs = []
        for file in documents:
            if not file.exists() or not file.is_file() or file.suffix != ".pdf":
                raise ValueError(f"El archivo '{file}' no es un PDF válido.")
            doc = self._process_document(file)
            clean_docs.append(doc)
        liberar_modelo_nlp()
        
        doc_matrix = self._vocabulary.transform(clean_docs)
        return self._model.decision_function(doc_matrix)