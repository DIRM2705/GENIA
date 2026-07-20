from utils.dataframe_utils import get_grouping_dataframe
from main import load_preprocessed_lf, lazy_from_csv, create_hipergraph
from preprocessing.psicometrical import extract_characteristics
from consts import REQUIRED_HG_COLUMNS, REQUIRED_OUTPUT_COLUMNS
from pathlib import Path
import polars as pl

def test_invalid_df():
    """
    Test loading invalid dataframes
    """
    
    try:
        lazy_from_csv(Path("data/test_data/non_existent_file.csv"))
    except FileNotFoundError as e:
        assert "no existe" in str(e)
    else:
        assert False, "Expected FileNotFoundError was not raised"
    
    try:
        load_preprocessed_lf(Path("data/test_data/invalid.parquet"))
    except FileNotFoundError as e:
        assert "no existe" in str(e)
    else:
        assert False, "Expected FileNotFoundError was not raised"
        
    try:
        load_preprocessed_lf(Path("data/test_data/preprocessing_test.csv"))
    except ValueError as e:
        assert "no es un archivo parquet" in str(e)
    else:
        assert False, "Expected ValueError was not raised"
    
def test_preprocess():
    """
    Test the preprocess of a valid csv file and its constraints
    """
    
    #This should be a valid df
    lf = lazy_from_csv(Path("data/test_data/preprocessing_test.csv"))
    df = extract_characteristics(lf)
    
    #Missing columns should raise a ValueError
    bad_df = df.drop("AN")
    try:
        extract_characteristics(bad_df)
    except ValueError as e:
        assert "son necesarias en el DataFrame" in str(e)
    else:
        assert False, "Expected ValueError was not raised"
        
    #Verify schema
    assert set(df.columns) == set(REQUIRED_OUTPUT_COLUMNS), f"The columns of the dataframe do not match the required columns."
    
    df.write_parquet(Path("data/test_data/preprocessed_test.parquet"))
    
def test_grouping_df():
    """
    Test the transformation of the dataframe so it can be used by the hypergraph constructor
    """
    
    grouping_df = load_preprocessed_lf(Path("data/test_data/preprocessed_test.parquet"))
    #This should be a valid df
    grouping_df = get_grouping_dataframe(grouping_df)
    
    #Verify that the grouping dataframe has the expected columns
    assert set(grouping_df.columns) == set(REQUIRED_HG_COLUMNS), f"The columns of the grouping dataframe do not match the expected columns."
    
    #Verify discretization of the columns
    for col in ["AN", "RN", "CN", "PL", "HS", "CE", "EE", "BE"]:
        assert grouping_df[col].dtype == pl.UInt8, f"The column {col} is not discretized to UInt8."
        assert grouping_df[col].max() < 5, f"The column {col} has values greater than or equal to 5, which is not expected after discretization."
        assert grouping_df[col].min() >= 0, f"The column {col} has values less than 0, which is not expected after discretization."
        
def test_hypergraph_construction():
    """
    Test the construction of the hypergraph from the grouping dataframe
    """
    #This should be a valid df
    grouping_df = load_preprocessed_lf(Path("data/test_data/synthetic_chars.parquet"))
    grouping_df = get_grouping_dataframe(grouping_df)
    
    create_hipergraph(grouping_df, Path("data/test_data/hypergraph_test.hg"))
    
def test_cognitive_load():
    """
    Test the cognitive load estimation for a set of subjects
    """
    
    from preprocessing.cognitive_load import Subject, get_cognitive_load_for_subjects
    
    subjects = [
        Subject("Matematicas", 5, 2.5, "logical thinking"),
        Subject("Inglés", 8, 4, "comunication"),
        Subject("Psicología", 6, 3, "humanities"),
        Subject("Ciencias Sociales", 7, 3.5, "social sciences")
    ]
    
    lf = pl.LazyFrame(
        {
            "Id": [1, 2, 3],
            "MI": [[0, 4], [1, 2], [3, 5]]
        }
    )
    df = get_cognitive_load_for_subjects(lf, subjects)
    
    #Verify that the cognitive load columns are present in the dataframe
    assert df.null_count().sum_horizontal().item() == 0, "There are null values in the cognitive load columns, which is not expected."
    
    #Verify that cognitive load columns behave as expected
    results = df.select(pl.exclude("Id", "MI")).to_numpy()
    assert results.shape == (3, 4), "The resulting dataframe should have 3 rows and 4 columns for the cognitive load of each subject."
    assert results[0][0] < results[1][0], "Cognitive load for Matematicas should be lower for student 1 than for student 2, given their MI."
    assert results[1][3] < results[0][3], "Cognitive load for Ciencias Sociales should be lower for student 2 than for student 1, given their MI."
    assert results[1][1] == results[2][1], "Cognitive load for Inglés should be equal for student 2 than for student 3, given their MI."
    assert results[2][1] < results[0][1], "Cognitive load for Inglés should be lower for student 3 than for student 1, given their MI."