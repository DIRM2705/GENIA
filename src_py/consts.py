VISUAL_IDX = 0
AURAL_IDX = 1
READWRITE_IDX = 2
KINESTHETIC_IDX = 3
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
        "MIKin", #corporal-cinestésica
        "MIExis", #Existencial
        "MIInter", #interpersonal
        "MIIntra", #intrapersonal
        "MILog", #logico-matemática
        "MIMus", #musical
        "MINat", #naturalista
        "MIVer", #verbal
        "MIVis" #visual (Inteligencia múltiple)
    ]

VARK_COLUMNS = [
    "VARKVisual",
    "VARKAural",
    "VARKReadWrite",
    "VARKKinesthetic"]


#Characteristic constants

ID_IDX  = 0
AM1_IDX  = 3
AM2_IDX  = 4
AM3_IDX  = 5
AM4_IDX  = 6
AM5_IDX  = 7
RM1_IDX  = 8
RM2_IDX  = 9
RM3_IDX  = 10
RM4_IDX  = 11
RM5_IDX  = 12
CM1_IDX  = 13
CM2_IDX  = 14
CM3_IDX  = 15
CM4_IDX  = 16
CM5_IDX  = 17
BE1_IDX  = 18
BE2_IDX  = 19
BE3_IDX  = 20
BE4_IDX  = 21
BE5_IDX  = 22
EE1_IDX  = 23
EE2_IDX  = 24
EE3_IDX  = 25
EE4_IDX  = 26
EE5_IDX  = 27
CE1_IDX  = 28
CE2_IDX  = 29
CE3_IDX  = 30
CE4_IDX  = 31
CE5_IDX  = 32
VARK_VIS_IDX  = 33
VARK_AUR_IDX  = 34
VARK_RW_IDX  = 35
VARK_KIN_IDX  = 36
MI_KIN_IDX  = 37
MI_EXIST_IDX  = 38
MI_INTER_IDX  = 39
MI_INTRA_IDX  = 40
MI_LOG_IDX  = 41
MI_MUS_IDX  = 42
MI_NAT_IDX  = 43
MI_VER_IDX  = 44
MI_VIS_IDX  = 45
