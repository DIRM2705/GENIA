import fitz
import spacy

# ==========================================================
# Extraer texto
# ==========================================================

def extraer_texto_pdf(ruta_pdf):
    texto = []

    with fitz.open(ruta_pdf) as doc:
        for pagina in doc:
            texto.append(pagina.get_text())

    return " ".join(texto)


# ==========================================================
# Procesar texto
# ==========================================================

nlp = spacy.load("es_core_news_sm")

def procesar_texto(texto):
    doc = nlp(texto)

    return [
        token.lemma_.lower()
        for token in doc
        if not token.is_stop
        and not token.is_punct
        and not token.is_space
        and not token.like_url
    ]
