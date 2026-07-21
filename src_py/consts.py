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

REQUIRED_INPUT_COLUMNS = [
    "Id",
    "Chronotype", #Cronotipo
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

REQUIRED_OUTPUT_COLUMNS = REQUIRED_INPUT_COLUMNS + ["VARK", "MI", "EGO", "IGO", "SE", "TV", "TA"]

REQUIRED_INPUT_COLUMNS.extend(MI_COLUMNS)
REQUIRED_INPUT_COLUMNS.extend(VARK_COLUMNS)

REQUIRED_HG_COLUMNS = ["Chronotype", "AN", "RN", "CN", "PL", "HS", "CE", "EE", "BE", "VARK", "MI"]

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

MI_INDICES = {
    "MIKin": 0,
    "MIExis": 1,
    "MIInter": 2,
    "MIIntra": 3,
    "MILog": 4,
    "MIMus": 5,
    "MINat": 6,
    "MIVer": 7,
    "MIVis": 8
}