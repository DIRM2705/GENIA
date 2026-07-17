from pathlib import Path #Para la ruta de los archivos
from typing import Any  #Para decir que se puede usar cualquier tipo de dato en una función o variable

import matplotlib
import matplotlib.pyplot as plt ### pyplot es el módulo encargado de crear las figuras
import numpy as np
import polars as pl

from consts import *

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

# Motor de renderizado de gráficos 2D que se utiliza para generar imágenes en memoria
matplotlib.use("Agg") 

default_directory = Path(".") # "." representa el directorio actual -> por si no se especifica donde, se guararan las imágenes ahí

##Construye un diccionario con la información del estudiante o del promedio del grupo, para luego usarlo en las funciones de graficación -> para así no tener que pasar el DataFrame completo a cada función de graficar
def info_graphics_fromDataframe(df: pl.DataFrame, selected_id: int | None = None, view_mode: str = "person",) -> dict[str, Any]:
    ids_availables = [int(value) for value in df["Id"].to_list() if value is not None]
    
    if selected_id is None:
        selected_id = ids_availables[0] #Si el usuario no especifica un Id, se toma el primero disponible

    row = None #no existe un estudiante específico, se tomará el promedio del grupo ->fila vacia
    
    if view_mode == "person":
        dataframe_filtered = df.filter(pl.col("Id") == int(selected_id)) # df.filter() -> Filtra la fila del Dataframe según la condicion -> si el id seleccionado es igual al id de la columna "Id" del Dataframe
        if dataframe_filtered.height == 0: # .height -> Devuelve el número de filas del DataFrame -> si es 0, significa que no se encontró ningún estudiante con ese id
            raise ValueError(f"No existe el estudiante con ID: {selected_id}.")
        row = dataframe_filtered.to_dicts()[0] #to_dicts() -> Convierte el DataFrame en una lista de diccionarios -> [0] -> toma el primer diccionario de la lista, que corresponde al estudiante seleccionado

    return {
        "selected_id": "promedio" if view_mode == "average" else int(selected_id),
        "view_mode": view_mode,
        "available_ids": ids_availables,
        "im_scores": _get_metric_vector_from_group_or_row(df=df, row=row, categories=INTELLIGENCE_BY_INDEX, raw_column="MI",), #Si row es None, se calculará el promedio del grupo, si no, se tomará el estudiante seleccionado ->genera lista para usar para la grafica de radar
        "vark_scores": _get_metric_vector_from_group_or_row(df=df, row=row, categories=VARK_BY_INDEX, raw_column="VARK",), #->genera lista para usar para la grafica de radar
        "cronotipo": _get_chronotype_data(df=df, row=row, view_mode=view_mode),
        "engagement": _get_group_or_row_profile(df=df, row=row, columns=["BE", "EE", "CE"],),
        "motivation": _get_group_or_row_profile(df=df, row=row, columns=["Orientacion_metas_intrinsecas", "Autoeficacia", "Valor_tarea", "Ansiedad_examenes"],),
        "labels": { #para graficas de RADAR
            "im": [IM_DISPLAY_LABELS[label] for label in INTELLIGENCE_BY_INDEX],
            "vark": [VARK_DISPLAY_LABELS[label] for label in VARK_BY_INDEX],},
    }

##Recibe la ruta donde se guardarán las imágenes y devuelve la ruta convertida en un objeto Path -> para que sea más fácil trabajar con rutas de archivos
def _resolve_download_direction(download_direction: str | Path | None) -> Path:
    if download_direction is None:
        return default_directory.expanduser().resolve() #Si no se especifica una ruta, se toma el directorio actual como ruta por defecto -> expanduser() -> Expande el símbolo ~ a la ruta del directorio home del usuario -> resolve() -> Devuelve la ruta absoluta del directorio actual

    candidate_path = Path(str(download_direction).strip().strip('"')).expanduser()
    if not candidate_path.is_absolute(): #Si la ruta especificada es no absoluta, manda error -> is_absolute() -> Devuelve True si la ruta es absoluta, False si es relativa
        raise ValueError("La ruta debe ser absoluta. "
                         "Ejemplo: C:\\Users\\Usuario\\Desktop\\Graficas")
 
    return candidate_path.resolve() # Normaliza la ruta y devuelve la ruta absoluta

##Genera y guarda las imágenes PNG
def save_images(df: pl.DataFrame, selected_id: int | None = None, view_mode: str = "person", download_direction: str | Path | None = None, dpi: int = 180,) -> list[str]: #dpi es la resolucion de la imgen -> devuelve una lista con las rutas de las imágenes generadas
    view_mode = view_mode.lower()
    # Si no es un estudiante individual,no existe un Id seleccionado
    if view_mode != "person":
        selected_id = None

    graphics_data = info_graphics_fromDataframe(df=df, selected_id=selected_id, view_mode=view_mode) #-> diccionario con la información del estudiante o del promedio del grupo
    export_dir = _resolve_download_direction(download_direction)
    export_dir.mkdir(parents=True, exist_ok=True) #Crea la carpeta si no existe -> parents=True -> Crea los directorios padres si no existen, exist_ok=True -> No lanza error si el directorio ya existe

    base_name = "promedio" if view_mode == "average" else f"estudiante_{int(selected_id)}"
    saved_files: list[str] = []

    #GRÁFICA DE RADAR DE INTELIGENCIAS MÚLTIPLES
    im_fig = plt.figure(figsize=(8, 6)) #Crea una nueva figura de 8x6 pulgadas
    ax_im = im_fig.add_subplot(111, projection="polar") #ax es eje de coordenadas -> 111 -> 1 fila, 1 columna, 1er subplot -> projection="polar" -> grafica polar
    _draw_radar_chart(ax=ax_im, labels=graphics_data["labels"]["im"], values=graphics_data["im_scores"], title="Inteligencias múltiples")
    im_path = export_dir / f"{base_name}_inteligencias_multiples.png"
    im_fig.savefig(im_path, format="png", dpi=dpi, bbox_inches="tight")
    plt.close(im_fig) #Se cierra la figura para liberar memoria y evitar que se acumulen figuras abiertas en memoria
    saved_files.append(str(im_path))

    #GRÁFICA DE RADAR DE VARK
    vark_fig = plt.figure(figsize=(8, 6)) #Crea una nueva figura de 8x6 pulgadas
    ax_vark = vark_fig.add_subplot(111, projection="polar")
    _draw_radar_chart(ax=ax_vark, labels=graphics_data["labels"]["vark"], values=graphics_data["vark_scores"], title="VARK")
    vark_path = export_dir / f"{base_name}_vark.png"
    vark_fig.savefig(vark_path, format="png", dpi=dpi, bbox_inches="tight")
    plt.close(vark_fig)
    saved_files.append(str(vark_path))

    #HISTOGRAMA DE CRONOTIPO
    cronotipo_fig = plt.figure(figsize=(8, 6))
    ax_cronotipo = cronotipo_fig.add_subplot(111)
    _draw_histogram_chart(ax=ax_cronotipo, data=graphics_data)
    cronotipo_path = export_dir / f"{base_name}_cronotipo.png"
    cronotipo_fig.savefig(cronotipo_path, format="png", dpi=dpi, bbox_inches="tight")
    plt.close(cronotipo_fig)
    saved_files.append(str(cronotipo_path))

    #DIAGRAMA DE CAJA DE COMPROMISO ->bloxplot
    boxplot_fig_Eng = plt.figure(figsize=(8, 6))
    ax_box = boxplot_fig_Eng.add_subplot(111)
    _draw_boxplot_chart_Eng(ax=ax_box, data=graphics_data)
    boxplot_path = export_dir / f"{base_name}_compromiso.png"
    boxplot_fig_Eng.savefig(boxplot_path, format="png", dpi=dpi, bbox_inches="tight")
    plt.close(boxplot_fig_Eng)
    saved_files.append(str(boxplot_path))
    
    #DIAGRAMA DE CAJA DE MOTIVACIÓN ->bloxplot
    boxplot_fig_Mot = plt.figure(figsize=(8, 6))
    ax_box = boxplot_fig_Mot.add_subplot(111)
    _draw_boxplot_chart_Mot(ax=ax_box, data=graphics_data)
    boxplot_path = export_dir / f"{base_name}_motivacion.png"
    boxplot_fig_Mot.savefig(boxplot_path, format="png", dpi=dpi, bbox_inches="tight")
    plt.close(boxplot_fig_Mot)
    saved_files.append(str(boxplot_path))

    return saved_files

#Guarda una tupla con el modo de vista, el Id seleccionado y la ruta de descarga -> para luego usarla en la función save_images -> igual obtiene la información del estudiante o del promedio del grupo para luego usarla en las funciones de graficación
def prompt_download_options(df: pl.DataFrame) -> tuple[str, int | None, str]: #-> Devuelve una tupla con el modo de vista, el Id seleccionado y la ruta de descarga
    print("\nSeleccione el tipo de perfil del desee descargar los gráficos:")
    print("1) Promedio del grupo")
    print("2) Perfil de un estudiante por Id")
    option = input("Opción [1/2]: ").strip().lower()

    if option in {"1", "promedio"}:
        view_mode = "average"
        selected_id = None
    else:
        view_mode = "person"
        selected_id_input = input("Ingrese el Id del estudiante: ").strip()
        if not selected_id_input:
            raise ValueError("Debe ingresar un Id válido.")
        selected_id = int(selected_id_input)

    export_dir = input(
        "Ingrese la ruta donde desea guardar las imágenes (ej. C:\\Users\\TuUsuario\\Desktop\\Graficas):  "
    )
    if not export_dir:
        export_dir = str(default_directory)

    return view_mode, selected_id, export_dir

#con la tupla obtenida de prompt_download_options, llama a save_images para generar y guardar las imágenes -> luego imprime en consola las rutas de las imágenes generadas
def export_info_from_console(df: pl.DataFrame) -> list[str]:
    view_mode, selected_id, export_dir = prompt_download_options(df)
    saved_files = save_images(df=df, selected_id=selected_id, view_mode=view_mode,download_direction=export_dir,)

    print("\nImágenes exportadas correctamente:")
    for path in saved_files:
        print(f"- {path}")
    return saved_files

#Para Engagement y Motivation
#Obtiene valores numericos de un estudiante especifico o el promedio (row=None)
def _get_group_or_row_profile(df: pl.DataFrame, row: dict[str, Any] | None, columns: list[str],) -> dict[str, float]: #-> columns es la lista de columnas que se quieren obtener -> Ejemplo: ["BE", "EE", "CE"] -> devuelve un diccionario con los valores de las columnas especificadas
    if row is None: #Si no se especifica un estudiante, se calcula el promedio del grupo -> devuelve un diccionario con los valores promedio de las columnas especificadas
        return {column: float(df[column].mean()) for column in columns if column in df.columns}
    values = {}
    for column in columns: #Si se especifica un estudiante, se obtienen los valores de las columnas especificadas para ese estudiante -> devuelve un diccionario con los valores de las columnas especificadas para el estudiante seleccionado
        values[column] = float(row.get(column, 0.0)) if row.get(column) is not None else 0.0
    return values

#Para VARK y MI
#Prepara los vectores de valores para un estudiante especifico o el promedio (row=None) -> para luego usarlos en las funciones de graficación
def _get_metric_vector_from_group_or_row(df: pl.DataFrame, row: dict[str, Any] | None, categories: list[str], raw_column: str,) -> list[float]:
    if row is None:
        vectors = []
        for item in df[raw_column].to_list() if raw_column in df.columns else []:
            if isinstance(item, list):
                selected_indices = {int(entry) for entry in item}
                vectors.append([1.0 if idx in selected_indices else 0.0 for idx in range(len(categories))])
            else:
                vectors.append([0.0 for _ in categories])
        if vectors:
            return [sum(vector[idx] for vector in vectors) / len(vectors) for idx in range(len(categories))]
        return [0.0 for _ in categories]

    if raw_column in row and isinstance(row[raw_column], list):
        selected_indices = {int(entry) for entry in row[raw_column]}
        return [1.0 if idx in selected_indices else 0.0 for idx in range(len(categories))]

    return [0.0 for _ in categories]

#Prepara los datos del cronotipo para un estudiante especifico o el promedio (row=None) -> para luego usarlos en las funciones de graficación
def _get_chronotype_data(df: pl.DataFrame, row: dict[str, Any] | None, view_mode: str) -> dict[str, Any]:
    counts = {"Matutino": 0, "Vespertino": 0}
    normalized_values = [_normalize_cronotipo(value) for value in df["Cronotipo"].to_list()]

    for label in normalized_values:
        counts[label] = counts.get(label, 0) + 1

    if row is None:
        selected_value = max(counts, key=counts.get)
    else:
        selected_value = _normalize_cronotipo(row.get("Cronotipo"))
    return {"group_counts": counts, "selected_value": selected_value, "values":normalized_values}

def _normalize_cronotipo(value: Any) -> str:
    if value is None:
        return "Sin datos"
    if isinstance(value, str):
        text = value.strip().lower()
        if "matut" in text or "7 am" in text:
            return "Matutino"
        if "vesp" in text or "3pm" in text:
            return "Vespertino"
        return value.strip()
    try:
        numeric_value = float(value)
    except (TypeError, ValueError):
        return str(value)
    return "Matutino" if numeric_value >= 2 else "Vespertino"

#DIBUJAR GRÁFICA DE RADAR
def _draw_radar_chart(ax: plt.Axes, labels: list[str], values: list[float], title: str) -> None:
    angles = np.linspace(0, 2 * np.pi, len(labels), endpoint=False).tolist() #Crea un array de ángulos equidistantes para cada etiqueta -> np.linspace()  Devuelve números espaciados uniformemente en un intervalo especificado -> 0 a 2*pi -> len(labels) -> número de etiquetas -> endpoint=False -> no incluye el último valor del intervalo
    angles += angles[:1] #Cierra el círculo de la gráfica de radar -> agrega el primer ángulo al final del array de ángulos para cerrar el círculo
    numeric_values = [float(value) for value in values] #convierte los valores a float para que sean compatibles con matplotlib
    numeric_values += numeric_values[:1] #Cierra el círculo de la gráfica de radar -> agrega el primer valor al final del array de valores para cerrar el círculo

    ax.set_theta_offset(np.pi / 2)
    ax.set_theta_direction(-1) #-1 es sentido horario, 1 es sentido antihorario
    ax.set_xticks(angles[:-1])
    ax.set_xticklabels(labels, fontsize=8)
    ax.set_ylim(0, 1.0)
    ax.plot(angles, numeric_values, color="#ae3140", linewidth=1.8) #.plot() -> Dibuja la línea de la gráfica de radar
    ax.fill(angles, numeric_values, color="#ea9496", alpha=0.25) #.fill() -> Rellena el área bajo la línea de la gráfica de radar
    ax.set_title(title, fontsize=10)

#DIUBUJAR HISTOGRAMA
def _draw_histogram_chart(ax: plt.Axes, data: dict[str, Any]) -> None:
    view_mode = data.get("view_mode", "average")
    selected_value = data["cronotipo"]["selected_value"]
    labels = ["Matutino", "Vespertino"]
    label_to_index = {label: i for i, label in enumerate(labels)}
    colors_group = ["#7c3aed", "#d946ef"]

    if view_mode == "person":
        # Un solo valor: el cronotipo de este estudiante
        raw_values = [selected_value] if selected_value in label_to_index else []
    else:
        # Todos los valores del grupo
        raw_values = data["cronotipo"]["values"]

    numeric_values = [label_to_index[v] for v in raw_values if v in label_to_index]

    bins = np.arange(len(labels) + 1) - 0.5
    counts_arr, _, patches = ax.hist(numeric_values, bins=bins, alpha=0.85, edgecolor="white")
    for index, patch in enumerate(patches):
        patch.set_facecolor(colors_group[index % len(colors_group)])

    ax.set_xticks(range(len(labels)))
    ax.set_xticklabels(labels)
    ax.set_ylabel("Frecuencia")

    ax.set_title("Cronotipo")
    

#DIBUJAR DIAGRAMA DE CAJA DE COMPROMISO
def _draw_boxplot_chart_Eng(ax: plt.Axes, data: dict[str, Any]) -> None:
    engagement_values = [data["engagement"][key] for key in ["BE", "EE", "CE"]]

    data = [engagement_values]
    labels = ["Compromiso"]
    bp = ax.boxplot(data, patch_artist=True)
    for box in bp["boxes"]:
        box.set(facecolor="#DFCC38", alpha=0.7)

    ax.set_xticks([1])
    ax.set_xticklabels(labels)
    ax.set_title("Compromiso")
    ax.set_ylabel("Puntaje")

#DIBUJAR DIAGRAMA DE CAJA DE MOTIVACIÓN
def _draw_boxplot_chart_Mot(ax: plt.Axes, data: dict[str, Any]) -> None:
    motivation_values = [data["motivation"][key] for key in ["Orientacion_metas_intrinsecas", "Autoeficacia", "Valor_tarea", "Ansiedad_examenes"]]

    data = [motivation_values]
    labels = ["Motivación"]
    bp = ax.boxplot(data, patch_artist=True)
    for box in bp["boxes"]:
        box.set(facecolor="#B1A4FF", alpha=0.7)

    ax.set_xticks([1])
    ax.set_xticklabels(labels)
    ax.set_title("Motivación")
    ax.set_ylabel("Puntaje")