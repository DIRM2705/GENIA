MI_COLUMNS = [
    "MIKin", #Cinestésica
    "MIExis", #Existencial
    "MIInter", #interpersonal
    "MIIntra", #intrapersonal
    "MILog", #logico-matemática
    "MIMus", #musical
    "MINat", #naturalista
    "MIVer", #verbal
    "MIVis" #visual
    ]

VARK_COLUMNS = [
    "VARKVisual", #Visual
    "VARKAural", #Auditivo
    "VARKReadWrite", #Lectura/Escritura
    "VARKKinesthetic" #Cinestésico
]

REQUIRED_COLUMNS = [
    "Id",
    "Cronotype", #Cronotipo
    "AN", #Necesidad de autonomía
    "RN", #Necesidad de relaciones
    "CN", #Necesidad de competencia
    "BE", #Compromiso conductual
    "EE", #Compromiso emocional
    "CE", #Compromiso cognitivo
    "HS", #Búsqueda de ayuda
    "PL", #Aprendizaje por pares
    "TM", #Manejo del tiempo
    "RH", #Repetición
    "EL", #Elaboración
    "OR", #Organización
    "CP", #Pensamiento crítico
    "MC"  #Metacognición
]
REQUIRED_COLUMNS.extend(MI_COLUMNS)
REQUIRED_COLUMNS.extend(VARK_COLUMNS)

IM_DISPLAY_LABELS = {
    "MIKin": "Cinestésica",
    "MIExis": "Existencial",
    "MIInter": "Interpersonal",
    "MIIntra": "Intrapersonal",
    "MILog": "Lógico-matemática",
    "MIMus": "Musical",
    "MINat": "Naturalista",
    "MIVer": "Verbal",
    "MIVis": "Visual",
}

VARK_DISPLAY_LABELS = {
    "VARKVisual": "Visual",
    "VARKAural": "Auditivo",
    "VARKReadWrite": "Lectura/Escritura",
    "VARKKinesthetic": "Kinestésico",
}