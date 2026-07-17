from utils.dataframe_utils import preprocess_from_csv
from preprocessing.dataframe import preprocess
from consts import REQUIRED_OUTPUT_COLUMNS
from pathlib import Path

def test_non_existent_file():
    """
    Test looking for a non existent file, should raise a FileNotFoundError
    """
    try:
        preprocess_from_csv(Path("src_py/tests/test_data/non_existent_file.csv"))
    except FileNotFoundError as e:
        assert "no existe" in str(e)
    else:
        assert False, "Expected FileNotFoundError was not raised"

def test_preprocess():
    """
    Test the preprocess of a valid csv file and its constraints
    """
    
    #This should be a valid df
    df = preprocess_from_csv(Path("src_py/tests/test_data/preprocessing_test.csv"))
    
    #Missing columns should raise a ValueError
    bad_df = df.drop("AN")
    try:
        preprocess(bad_df.lazy())
    except ValueError as e:
        assert "son necesarias en el DataFrame" in str(e)
    else:
        assert False, "Expected ValueError was not raised"
        
    #Verify schema
    assert set(df.columns) == set(REQUIRED_OUTPUT_COLUMNS), f"The columns of the dataframe do not match the required columns. Expected: {REQUIRED_OUTPUT_COLUMNS}, got: {df.columns}"