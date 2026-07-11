from pathlib import Path

import fitz
import spacy
from unicodedata import normalize
from pathlib import Path

nlp = None

def token_valido(token : spacy.tokens.Token) -> bool:
    """_summary_
    Función que recibe una palabra y devuelve si es válida o no
    Args:
        palabra (str): Palabra a evaluar
    Returns:
        bool: True si es válida, False en caso contrario
    """
    return (token.is_alpha or token.is_digit) and not token.is_stop and not token.is_punct and not token.is_space and not token.like_url

def normalizar_lema(lema : str) -> str:
    """_summary_
    Función que recibe un lema y devuelve su forma normalizada
    Args:
        lema (str): Lema a normalizar
    Returns:
        str: Lema normalizado
    """
    return normalize("NFKD", lema).encode("ASCII", "ignore").decode("utf-8").lower()

# ==========================================================
# Extraer texto
# ==========================================================

def _extraer_texto_pdf(ruta_pdf : Path) -> list[str]:
    texto = []

    with fitz.open(ruta_pdf) as doc:
        for pagina in doc:
            texto.append(pagina.get_text())
            
    return texto


# ==========================================================
# Procesar texto
# ==========================================================

def _procesar_texto(lineas : list[str]) -> list[str]: 
    lemmas = set()
    for texto in lineas:
        texto = normalizar_lema(texto)
        doc = nlp(texto)
        lemmas.update(
            normalizar_lema(token.lemma_) for token in doc if token_valido(token)
        )
    
    return list(lemmas)

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
    