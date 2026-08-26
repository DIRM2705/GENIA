from genia_libs.utils.dataframe import load_preprocessed_lf
from genia_libs.preprocessing.needs import make_faculty_recommendations

if __name__ == "__main__":
    # Cargar el LazyFrame preprocesado y convertir a dataframe de polars
    lf = load_preprocessed_lf("data/test_data/Psychoeducational_Features_for_Group_Forming.parquet")
    lf = lf.collect().sample(n=25, seed=42).lazy() # Elige a 25 alumnos al azar para crear un grupo de prueba
    
    # Generar recomendaciones para el profesorado
    recommendations = make_faculty_recommendations(lf, n_components=3)

    # Imprimir las recomendaciones
    for rec in recommendations:
        print(rec)