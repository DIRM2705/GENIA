from genia_libs.utils.visualization import save_images
from genia_libs.utils.dataframe import load_preprocessed_lf

if __name__ == "__main__":
    # Carga el lazyframe preprocesado y lo convierte en un dataframe de polars
    lf = load_preprocessed_lf("data/test_data/Psychoeducational_Features_for_Group_Forming.parquet")
    df = lf.collect()
    
    id = 42 #Elige un ID de estudiante para visualizar sus características
    
    #Guarda las imágenes de las características del grupo en la carpeta "results/"
    image_path = "results/"
    save_images(df, selected_id=id, download_direction=image_path, view_mode="person")