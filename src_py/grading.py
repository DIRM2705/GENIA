import polars as pl
from group_enhancer import PyStudent
from consts import *

def grade_students(students : pl.DataFrame) -> pl.DataFrame:
    """
    Given a student record, return a PyStudent object with the results of the tests
    
    Args:
        student (pl.Row): A Polars Row representing a student with at least the following fields:
            - "ID": Unique identifier for the student
            - "Cronotype": The student's cronotype
            - "TND": The student's Neurodevelopmental Disorder status regarding
                     ADHD, ADD, ASD, Dislexia, Disgraphia and Discalculia
            - "IM1": The student's answers to the first multiple intelligence set
            - "IM2": The student's answers to the second multiple intelligence set
            - "IM3": The student's answers to the third multiple intelligence set
            - "VARK 8-20": The student's answers to the VARK questionnaire
            - "Engagement 21-35: The student's answers to the engagement questionnaire
            - "Motivation 36-43": The student's answers to the motivation questionnaire
            
    Returns:
        DataFrame: A Polars DataFrame with the data obtained by grading student's formularies
    """
    VARK_scores = get_VARK_scores(students.select([f"VARK{i}" for i in range(1,14)]))
    students = students.with_columns(
        TND = get_NDD_bitmask(students["TND"]),
        AM = (pl.col("AM1") + pl.col("AM2") + pl.col("AM3"))/3,
        RM = (pl.col("RM1") + pl.col("RM2") + pl.col("RM3"))/3,
        CM = (pl.col("CM1") + pl.col("CM2"))/2
    ).select([
        "Id", "Cronotipo", "TND", "AM", "RM", "CM"
    ])
    
    students = students.hstack(VARK_scores)
    
    return students
    
        
        
            
def get_NDD_bitmask(tnd_series : pl.Series) -> pl.Series:
    """
    Given the string of NDD diagnostics, convert them to a bitmask

    Args:
        tnd_string (str): String containing the NDD diagnostics separated by semicolons.

    Returns:
        int: Bitmask representing the presence of NDD diagnostics
    """
    
    aux_df = pl.DataFrame()
    aux_df = aux_df.with_columns(
        answers = tnd_series.str.to_lowercase().str.split(';'),
        TND = pl.Series("TND", [0]*len(tnd_series), dtype=pl.Int8)
    )
    
    for i in range(len(NDD_LIST)):
        ndd = NDD_LIST[i]
        aux_df = aux_df.with_columns(
            TND = pl.when(
                pl.col("answers").list.contains(ndd)
            ).then(
                pl.col("TND") | (1 << i)
            ).otherwise(
                pl.col("TND")
            )
        )

    return aux_df["TND"]

def get_IM_scores(im_answers: list[str], answer_list : list[str]) -> dict[str, int]:
    """
    Given a list of answers to the Multiple Intelligences questionnaire,
    return a dictionary with the scores for each intelligence type.
    
    Args:
        im_answers (list[str]): List of answers to the Multiple Intelligences questionnaire.
    """
    raise NotImplementedError("Function not yet implemented")

def get_VARK_scores(vark_answers: pl.DataFrame) -> pl.DataFrame:
    """
    Given a list of answers to the VARK questionnaire,
    return a dictionary with the scores for each VARK type.
    
    Args:
        vark_answers (list[str]): List of answers to the VARK questionnaire.
    """
    
    vark_answers = vark_answers.with_columns(
        Answers=pl.concat_list([pl.col(f"VARK{i}").str.to_lowercase().str.split(";") for i in range(1,14)]),
    ).select("Answers")    
    
    vark_answers = vark_answers.with_columns(
        Visual = pl.col("Answers").list.set_intersection(VISUAL_ANSWERS).list.len()/len(VISUAL_ANSWERS),
        Aural = pl.col("Answers").list.set_intersection(AURAL_ANSWERS).list.len()/len(AURAL_ANSWERS),
        ReadWrite = pl.col("Answers").list.set_intersection(READ_WRITE_ANSWERS).list.len()/len(READ_WRITE_ANSWERS),
        Kinesthetic = pl.col("Answers").list.set_intersection(KINESTHETIC_ANSWERS).list.len()/len(KINESTHETIC_ANSWERS)
    )
    
    return vark_answers.select(["Visual", "Aural", "ReadWrite", "Kinesthetic"])