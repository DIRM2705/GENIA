import polars as pl
from group_enhancer import PyStudent

def grade_student(student : pl.Row) -> PyStudent:
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
        PyStudent: An object containing the student's ID and their test results
    """
    
    #cronotype = 
    ndd = get_NDD_bitmask(student["TND"])
            
def get_NDD_bitmask(tnd_string: str) -> int:
    """
    Given the string of NDD diagnostics, convert them to a bitmask

    Args:
        tnd_string (str): String containing the NDD diagnostics separated by semicolons.

    Returns:
        int: Bitmask representing the presence of NDD diagnostics
    """
    if not tnd_string or tnd_string.strip() == "":
        return 0
    NDD_LIST = ['Disgrafía', 'Discalculia', 'Dislexia', 'TDA o TDAH', 'TEA']
    ndd = 0
    disorders = tnd_string.split(';')
    for i in range(len(NDD_LIST)):
        if NDD_LIST[i] in disorders:
            ndd |= 1 << i # Set the corresponding bit if the disorder is present
    return ndd

def get_IM_scores(im_answers: list[str], answer_list : list[str]) -> dict[str, int]:
    """
    Given a list of answers to the Multiple Intelligences questionnaire,
    return a dictionary with the scores for each intelligence type.
    
    Args:
        im_answers (list[str]): List of answers to the Multiple Intelligences questionnaire.
    """
    raise NotImplementedError("Function not yet implemented")

def get_VARK_scores(vark_answers: list[str]) -> dict[str, int]:
    """
    Given a list of answers to the VARK questionnaire,
    return a dictionary with the scores for each VARK type.
    
    Args:
        vark_answers (list[str]): List of answers to the VARK questionnaire.
    """
    VISUAL_ANSWERS = []
    