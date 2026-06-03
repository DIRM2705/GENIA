from dataclasses import dataclass

@dataclass
class Experiment:
    """
    Clase base para experimentos, cada experimento específico hereda de esta clase e 
    implementa su propio método run() para ejecutar el experimento.
    """
    
    _name : str
    _explanation : str
    _run_function : callable
    
    def __init__(self, name : str, explanation : str, run_function : callable):
        self._name = name
        self._explanation = explanation
        self._run_function = run_function

    def run(self, verbose : bool = False):
        print(f"Ejecutando experimento: {self._name}")
        if verbose:
            print(f"Descripción: {self._explanation}")
        self._run_function()