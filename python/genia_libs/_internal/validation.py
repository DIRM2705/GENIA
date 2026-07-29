from functools import wraps
from types import UnionType
import polars as pl
import inspect
from typing import get_type_hints, get_origin, get_args


def validate_columns(lf: pl.LazyFrame | pl.DataFrame, required_columns: list) -> None:
    """
    Verifica que el DataFrame tenga las columnas necesarias para el preprocesamiento
    
    Args:
        lf (pl.LazyFrame | pl.DataFrame): DataFrame a verificar
        
    Raises:
        ValueError: Si falta alguna columna necesaria
    """
    
    if isinstance(lf, pl.DataFrame):
        missing_columns = [col for col in required_columns if col not in lf.columns]
    else:
        missing_columns = [col for col in required_columns if col not in lf.collect_schema().names()]
    
    if missing_columns:
        raise ValueError(f"Las siguientes columnas no existen y son necesarias en el DataFrame: {missing_columns}")
    
def _is_parametrized_generic(type_hint) -> bool:
    """_summary_
    Check wether the type hint is a prameterized generic type (e.g. list[int], dict[str, int], etc.)

    Args:
        type_hint (_type_): The type hint to check
    """
    
    origin = get_origin(type_hint)
    args = get_args(type_hint)
    return origin is not None and args is not None and len(args) > 0 and origin is not UnionType

def _is_instance(obj, type_hint) -> bool:
    """_summary_
    Check wether the object is an instance of the type hint, including parameterized generic types (e.g. list[int], dict[str, int], etc.)

    Args:
        obj (_type_): The object to check
        type_hint (_type_): The type hint to check against
    """
    
    if _is_parametrized_generic(type_hint):
        origin_type = get_origin(type_hint)
        if not isinstance(obj, origin_type):
            return False
        
        if origin_type is list:
            args_type = get_args(type_hint)
            if args_type and not all(_is_instance(item, args_type[0]) for item in obj):
                return False
        return True
    else:
        return isinstance(obj, type_hint)
    
def validate_parameters(function : callable):
    """_summary_
    Decorator to validate functions receive the correct parameters types, and raise a TypeError if not.
    """
    @wraps(function)
    def wrapper(*args, **kwargs):
        # Validar que los parámetros sean del tipo esperado
        signature = inspect.signature(function)
        hints = get_type_hints(function)
        
        bound = signature.bind(*args, **kwargs)
        bound.apply_defaults()
        
        for name, real_type in bound.arguments.items():
            try:
                expected_type = hints[name]
                if not _is_instance(real_type, expected_type):
                    raise TypeError(f"El parámetro '{name}' debe ser del tipo {expected_type.__name__}, pero se recibió {type(real_type).__name__}")
            except KeyError:
                pass
        return function(*args, **kwargs)
    return wrapper