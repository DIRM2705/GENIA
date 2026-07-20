import polars as pl
from dataclasses import dataclass

from consts import MI_INDICES

@dataclass
class Subject:
    _name : str
    _interacting_elements : float
    _nem_area : str
    
    def __init__(self, name: str, num_units : int, avg_topics_per_unit : float, area : str):
        """
        Initializes a Subject instance.

        Parameters:
        name (str): The name of the subject.
        num_units (int): The number of units in the subject.
        avg_topics_per_unit (float): The average number of topics per unit.
        area (str): The area within the following [logical thinking, humanities, social sciences, comunication]
        to which the subject belongs.
        """
        self._name = name
        self._interacting_elements = num_units * avg_topics_per_unit
        self._nem_area = area
        
    @property
    def name(self) -> str:
        return self._name
    
    @property
    def interacting_elements(self) -> float:
        return self._interacting_elements
    
    @property
    def benefited_intelligences(self) -> set[str]:
        match self._nem_area:
            case "logical thinking":
                return {MI_INDICES["MILog"], MI_INDICES["MINat"], MI_INDICES["MIVis"], MI_INDICES["MIKin"]}
            case "humanities":
                return {MI_INDICES["MIKin"], MI_INDICES["MIInter"], MI_INDICES["MIIntra"], MI_INDICES["MIMus"]}
            case "social sciences":
                return {MI_INDICES["MIExis"], MI_INDICES["MIInter"], MI_INDICES["MINat"], MI_INDICES["MIIntra"]}
            case "comunication":
                return {MI_INDICES["MIVer"], MI_INDICES["MIMus"], MI_INDICES["MIInter"]}
            case _:
                raise ValueError(f"Area {self._nem_area} is not a valid area. Valid areas are: logical thinking, humanities, social sciences, comunication.")
        

def get_cognitive_load_for_subjects(students : pl.LazyFrame, subjects : list[Subject]) -> pl.DataFrame:
    """
    Estimates cognitive load for each student in the LazyFrame
    using the subjects in the dictionary.

    Parameters:
    students (pd.LazyFrame): The LazyFrame containing students top Multiple Intelligences
    subjects (list[Subject]): A list of Subject instances.
    

    Returns:
    pd.DataFrame: DataFrame with estimated cognitive load features.
    """
    students = students.select("Id", "MI")
    for subject in subjects:
        students = students.with_columns(
            (subject.interacting_elements/(1 +
                pl.col("MI").list.set_intersection(subject.benefited_intelligences)
                .list.len()*0.25
                )
            ) # interacting_elements/expertise
            .alias(f"{subject.name}_load"),
        )
    
    return students.collect()