from genia_libs.preprocessing.cognitive_load import Subject, get_cognitive_load_for_subjects
from genia_libs.utils.dataframe import load_preprocessed_lf

if __name__ == "__main__":
    # Definir las materias para los cuales se desea estimar la carga cognitiva
    subjects = [
        Subject(name="Matemáticas", num_units=10, avg_topics_per_unit=3, area="logical thinking"),
        Subject(name="Física", num_units=10, avg_topics_per_unit=3, area="logical thinking"),
        Subject(name="Psicología", num_units=10, avg_topics_per_unit=3, area="humanities"),
        Subject(name="Cs. Sociales", num_units=10, avg_topics_per_unit=3, area="social sciences"),
        Subject(name="Historia", num_units=10, avg_topics_per_unit=3, area="social sciences"),
    ]
    
    # Cargar el LazyFrame preprocesado y convertir a dataframe de polars
    lf = load_preprocessed_lf("data/test_data/Psychoeducational_Features_for_Group_Forming.parquet")
    df = lf.collect().sample(n=10, seed=42)  # Elige a 25 alumnos al azar para crear un grupo de prueba

    # Estimar la carga cognitiva para los sujetos definidos
    cognitive_loads = get_cognitive_load_for_subjects(df.lazy(), subjects)

    # Imprimir los resultados
    print("Estimación de la carga cognitiva para los sujetos:")
    print(cognitive_loads)