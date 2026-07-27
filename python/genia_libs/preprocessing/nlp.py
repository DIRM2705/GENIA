from pathlib import Path

import pypdfium2
import spacy
from unicodedata import normalize
from pathlib import Path

_nlp = None

def _token_valido(token : spacy.tokens.Token) -> bool:
    """_summary_
    Función que recibe una palabra y devuelve si es válida o no
    Args:
        palabra (str): Palabra a evaluar
    Returns:
        bool: True si es válida, False en caso contrario
    """
    return (token.is_alpha or token.is_digit) and not token.is_stop and not token.is_punct and not token.is_space and not token.like_url

def _normalizar_lema(lema : str) -> str:
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

    with pypdfium2.PdfDocument(ruta_pdf) as doc:
        for pagina in doc:
            texto.append(pagina.get_textpage().get_text_range())
            
    return texto

# ==========================================================
# Procesar texto
# ==========================================================

def _procesar_texto(lineas : list[str]) -> list[str]: 
    lemmas = set()
    for texto in lineas:
        texto = _normalizar_lema(texto)
        doc = _nlp(texto)
        lemmas.update(
            _normalizar_lema(token.lemma_) for token in doc if _token_valido(token)
        )
    
    return list(lemmas)

def cargar_modelo__nlp():
    global _nlp
    _nlp = spacy.load("es_core_news_sm") 

def liberar_modelo__nlp():
    global _nlp
    del _nlp
    _nlp = None

def procesar_pdf(path_pdf : Path) -> list[str]:
    if _nlp is None:
        raise TypeError("No se ha cargado el modelo de procesamiento de lenguaje natural")
    
    if not path_pdf.exists():
        raise FileNotFoundError("El archivo no existe")
    
    if path_pdf.is_dir() or path_pdf.suffix != ".pdf":
        raise FileNotFoundError("La ruta no es un archivo pdf")
    
    texto = _extraer_texto_pdf(path_pdf)
    return _procesar_texto(texto)
    