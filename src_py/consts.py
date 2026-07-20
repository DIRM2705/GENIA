NDD_LIST = ['disgrafía', 'discalculia', 'dislexia', 'tda o tdah', 'tea']

VISUAL_ANSWERS = [
        "dibujaría un mapa?",
        "le dibujen un mapa?",
        "le mostraría los detalles en un mapa del mundo?",
        "revisa las imágenes del libro de recetas en busca de ideas?",
        "les muestra diapositivas y fotografías de los parques?",
        "se ve de buena calidad",
        "representaciones visuales - imágenes, diagramas, tablas",
        "pictionary",
        "observa la palabra en su mente y escoge la opción que se ve mejor?",
        "se ve \"ok\"",
        "ver un tráiler de la película",
        "diagramas de flujo, tablas y diapositivas?"
    ]
    
AURAL_ANSWERS = [
        "daría instrucciones verbalmente?",
        "le den las direcciones por llamada telefónica?",
        "lo llamaría de inmediato y le contaría al respecto?",
        "busca consejo de alguien más?",
        "les da una charla/exposición sobre parques nacionales?",
        "la recomendación de un amigo",
        "escuchar la explicación de alguien más",
        "escucharlo sonar",
        "llamaría por teléfono a un amigo y le preguntaría sobre el programa?",
        "la pronuncia lenta y articuladamente en su mente?",
        "la opinión de un amigo",
        "que sus amigos hablen de la película",
        "discusión en grupos, conferencias?"
    ]
    
READ_WRITE_ANSWERS = [
        "escribiría las direcciones en un papel?",
        "le escriban las direcciones?",
        "le mandaría una copia impresa del itinerario?",
        "busca en un libro específico en el que hay una buena receta?",
        "les da un libro sobre parques nacionales?",
        "leer más detalles del producto",
        "instrucciones escritas",
        "scrabble",
        "leería el manual que viene en el programa?",
        "busca la escritura correcta en el diccionario?",
        "hojear partes del libro",
        "leer una reseña",
        "hojas de trabajo o libros de texto?"
    ]
    
KINESTHETIC_ANSWERS = [
        "la recogería de su hotel en su auto?",
        "lo recoja del hotel su amigo en su auto?",
        "cocina algo sin necesidad de instrucciones?",
        "los lleva a un parque nacional?",
        "haciendo esta nueva actividad",
        "caras y gestos",
        "le pediría a un amigo que le enseñe a usarlo?",
        "escribe ambas versiones?",
        "usar la copia del libro de un amigo",
        "investigación de campo, laboratorios, sesiones prácticas?"
    ]


ANSWER_LISTS = {
    "IM1": [
        "tengo un estilo de vida activo",
        "los ejercicios de meditación son satisfactorios",
        "me \"pongo la camiseta\" por el equipo",
        "la justicia es importante para mi",
        "estructurar mis ideas me ayuda a ser exitoso",
        "disfruto muchos tipos de música",
        "en mi casa tengo un sistema de reciclaje",
        "tengo un diario personal",
        "disfruto hacer rompecabezas tridimensionales",
    ],
    "IM2": [
        "disfruto los deportes y jugar afuera",
        "las preguntas sobre el significado de la vida son importantes para mí",
        "aprendo mejor interactuando con otros",
        "la injusticia social me preocupa",
        "me frustro fácilmente con personas desorganizadas",
        "siempre me ha interesado tocar un instrumento musical",
        "los animales son importantes en mi vida",
        "escribo por placer",
        "puedo recordar cosas en imágenes mentales",
    ],
    "IM3": [
        "me gusta trabajar con herramientas",
        "me gusta discutir cuestiones relacionadas a la vida",
        "actividades como los clubs escolares y las actividades extracurriculares son divertidas",
        "aprendo mejor cuando estoy involucrado emocionalmente con el tema",
        "las instrucciones paso a paso son una gran ayuda",
        "recordar letras de canciones es fácil para mí",
        "el senderismo es una actividad entretenida",
        "me interesan las lenguas extranjeras",
        "puedo ilustrar con imágenes las ideas en mi mente",
    ],
}

INTELLIGENCE_BY_INDEX = [
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

VARK_BY_INDEX = [
    "VARKVisual",
    "VARKAural",
    "VARKReadWrite",
    "VARKKinesthetic"]

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