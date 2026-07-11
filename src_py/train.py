from models.curriculum_classifier import CurriculumClassifier
from pathlib import Path
from dotenv import load_dotenv
import os

if __name__ == "__main__":
    load_dotenv()
    training_set_path = Path(os.getenv("TRAINING_CURRICULUM_PATH"))
    model = CurriculumClassifier()
    model.train(training_set_path)