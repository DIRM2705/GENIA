import joblib

if __name__ == "__main__":
    model = joblib.load("python/genia_libs/models/curriculum_classifier/curriculum_classifier.pkl")  # Cargar el modelo de carga cognitiva 
    predicciones = model.predict(["data/test_data/FISICA.pdf"])  # Predecir la carga cognitiva para el PDF procesado 
    
    print("Predicciones de area para los temarios:")
    print(predicciones)