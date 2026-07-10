from training.curriculum_classifier import train_curriculum_classifier
from pathlib import Path
from dotenv import load_dotenv
import os

if __name__ == "__main__":
    load_dotenv()
    training_set_path = Path(os.getenv("TRAINING_CURRICULUM_PATH"))
    model = train_curriculum_classifier(training_set_path)
    model.write_parquet(r"src_py\models\curriculm_vectors.parquet")
    
    print("Numero de lemas encontrados: ", model.shape)
    
    print(model.head(20))