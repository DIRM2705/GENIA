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

RECOMENDATIONS = {
    "BE": "Un profesor que mantenga una buena sintonía y manejo de grupo", #Compromiso conductual
    "EE": "Un profesor inspirador, apasionado por su materia",  #Compromiso emocional
    "CE": "Un profesor que cuente con amplios conocimientos de su tema y buena relación pedagógica", #Compromiso cognitivo
    "HS": "Un profesor con solidaridad pedagógica", #Búsqueda de ayuda
    "PL": "Un profesor que implemente el trabajo en equipo", #Aprendizaje por pares
    "TM": "Un profesor que calendarice y estipule fechas de entrega claras y precisas", #Manejo del tiempo
    "RH": "Un profesor que en la mayoría de sus sesiones tenga un método de enseñanza práctico", #Repetición
    "EL": "Un profesor que utilice estrategias que fomenten una estructura cognitiva", #Elaboración
    "OR": "Un profesor que presenta su contenido siguiendo una estructura cognitiva", #Organización
    "CP": "Un profesor que promueva los debates, discusiones y expresión de ideas", #Pensamiento crítico
    "MC": "Un profesor que fomente el autodidactismo",  #Metacognición
    "EGO": "Un profesor que fomente la competitividad en el grupo y exija cierto grado de participación de los estudiantes", 
    "IGO": "", 
    "SE": "", 
    "TV": "", 
    "TA": ""
}