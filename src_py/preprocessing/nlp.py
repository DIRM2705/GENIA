from pathlib import Path

import fitz
import spacy
from unicodedata import normalize
from pathlib import Path

nlp = None

# ==========================================================
# Extraer texto
# ==========================================================

def _extraer_texto_pdf(ruta_pdf : Path) -> str:
    texto = []

    with fitz.open(ruta_pdf) as doc:
        for pagina in doc:
            texto.append(pagina.get_text())

    return " ".join(texto)


# ==========================================================
# Procesar texto
# ==========================================================

def _procesar_texto(texto : str) -> list[str]:  
    doc = nlp(texto)
    lemmas = []
    
    for token in doc:
        if not token.is_stop and not token.is_punct and not token.is_space and not token.like_url:
            lemmas.append(
                normalize("NFKD", token.lemma_).encode("ASCII", "ignore").decode("utf-8").lower()
            )
            
    return lemmas

def cargar_modelo_nlp():
    global nlp
    nlp = spacy.load("es_core_news_sm") 

def liberar_modelo_nlp():
    global nlp
    del nlp
    nlp = None

def procesar_pdf(path_pdf : Path) -> list[str]:
    if nlp is None:
        raise TypeError("No se ha cargado el modelo de procesamiento de lenguaje natural")
    
    if not path_pdf.exists():
        raise FileNotFoundError("El archivo no existe")
    
    if path_pdf.is_dir() or path_pdf.suffix != ".pdf":
        raise FileNotFoundError("La ruta no es un archivo pdf")
    
    texto = _extraer_texto_pdf(path_pdf)
    return _procesar_texto(texto)
    