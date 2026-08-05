from __future__ import annotations

from pathlib import Path

import polars as pl
import pytest
import numpy as np

import genia_libs.preprocessing.needs as needs_module
from genia_libs.preprocessing.needs import _needs_analysis, make_faculty_recommendations
from genia_libs._internal.consts import REQUIRED_HG_COLUMNS, REQUIRED_INPUT_COLUMNS, REQUIRED_OUTPUT_COLUMNS
from genia_libs._internal.validation import validate_parameters
from genia_libs.preprocessing.cognitive_load import Subject, get_cognitive_load_for_subjects
from genia_libs.preprocessing.psicometrical import extract_characteristics
from genia_libs.utils.dataframe import get_grouping_dataframe, lazy_from_csv, load_preprocessed_lf


def _build_valid_student_frame() -> pl.LazyFrame:
    row = {name: [1] for name in REQUIRED_INPUT_COLUMNS if name != "Id"}
    row["Id"] = [1]
    return pl.DataFrame(row).lazy()


def test_validate_parameters_rejects_wrong_types():
    @validate_parameters
    def sample(a: int, b: str):
        return a, b

    with pytest.raises(TypeError, match="a"):
        sample("invalid", "ok")

    with pytest.raises(TypeError, match="b"):
        sample(1, 123)


def test_loading_files_raise_expected_errors(tmp_path: Path):
    with pytest.raises(FileNotFoundError, match="no existe"):
        lazy_from_csv(tmp_path / "missing.csv")

    with pytest.raises(FileNotFoundError, match="no existe"):
        load_preprocessed_lf(tmp_path / "missing.parquet")

    invalid_csv = tmp_path / "invalid.csv"
    invalid_csv.write_text("a,b\n1,2\n", encoding="utf-8")
    with pytest.raises(ValueError, match="no es un archivo parquet"):
        load_preprocessed_lf(invalid_csv)


def test_extract_characteristics_success_and_missing_columns():
    valid_lf = _build_valid_student_frame()
    result = extract_characteristics(valid_lf)

    assert set(result.columns) == set(REQUIRED_OUTPUT_COLUMNS)

    bad_lf = valid_lf.drop("AN")
    with pytest.raises(ValueError, match="son necesarias"):
        extract_characteristics(bad_lf)


def test_grouping_dataframe_discretizes_columns():
    data = {col: [0.1, 0.8, 1., 0.25, 0.9] for col in REQUIRED_HG_COLUMNS if col != "Id"}
    data["Id"] = [1, 2, 3, 4, 5]

    df = pl.DataFrame(data)
    grouping_df = get_grouping_dataframe(df)

    assert set(grouping_df.columns) == set(REQUIRED_HG_COLUMNS)

    for col in ["AN", "RN", "CN", "BE", "EE", "CE", "HS", "PL"]:
        assert grouping_df[col].dtype == pl.UInt8
        assert grouping_df[col].min() >= 0
        assert grouping_df[col].max() < 5


def test_cognitive_load_estimation():
    subjects = [
        Subject("Matematicas", 5, 2.5, "logical thinking"),
        Subject("Inglés", 8, 4, "comunication"),
        Subject("Psicología", 6, 3, "humanities"),
        Subject("Ciencias Sociales", 7, 3.5, "social sciences"),
    ]

    lf = pl.LazyFrame(
        {
            "Id": [1, 2, 3, 4, 5],
            "MI": [[0, 4], [1, 2], [3, 5], [2, 3], [4, 5]],
        }
    )
    df = get_cognitive_load_for_subjects(lf, subjects)

    assert df.null_count().sum_horizontal().item() == 0

    results = df.select(pl.exclude("Id", "MI")).to_numpy()
    assert results.shape == (5, 4)


def test_visualization_helper_paths():
    from genia_libs.utils import visualization as viz

    assert viz._resolve_download_direction(None) == Path(".").absolute()
    assert viz._resolve_download_direction("out") == Path("out").absolute()
    assert viz._resolve_download_direction(Path("out")).is_absolute()


def test_nlp_helpers_and_pdf_processing(tmp_path: Path, monkeypatch):
    try:
        import genia_libs.preprocessing.nlp as nlp_module
    except Exception as exc:  # pragma: no cover
        pytest.skip(f"NLP dependency unavailable: {exc}")

    assert nlp_module._normalizar_lema("Ángel") == "angel"

    class TokenStub:
        def __init__(self, *, alpha=True, digit=False, stop=False, punct=False, space=False, url=False, lemma=""):
            self.is_alpha = alpha
            self.is_digit = digit
            self.is_stop = stop
            self.is_punct = punct
            self.is_space = space
            self.like_url = url
            self.lemma_ = lemma

    assert nlp_module._token_valido(TokenStub(alpha=True, stop=False, punct=False, space=False, url=False))
    assert not nlp_module._token_valido(TokenStub(alpha=True, stop=True))

    nlp_module.cargar_modelo_nlp()
    result = nlp_module._procesar_texto(["Hola, mundo", "mundo feliz"])

    assert "hola" in result
    assert "mundo" in result
    assert "feliz" in result
    nlp_module.liberar_modelo_nlp()

class FakePCA:
    def __init__(self, n_components: int):
        self.n_components = n_components

    def fit(self, data):
        feature_count = len(data.collect_schema().names())
        self.components_ = np.zeros((self.n_components, feature_count), dtype=float)

        for idx in range(min(self.n_components, feature_count)):
            self.components_[idx, idx] = 1.0

        weights = np.arange(self.n_components, 0, -1, dtype=float)
        self.explained_variance_ratio_ = weights / weights.sum()
        return self


def test_needs_analysis_returns_ranked_features(monkeypatch):
    captured = {}

    def fake_validate_columns(lf, required_columns):
        captured["lf"] = lf
        captured["required_columns"] = required_columns

    monkeypatch.setattr(needs_module, "validate_columns", fake_validate_columns)
    monkeypatch.setattr(needs_module, "PCA", FakePCA)

    lf = pl.DataFrame(
        {
            "feature_a": [1, 2, 3, 4],
            "feature_b": [4, 3, 2, 1],
            "feature_c": [2, 2, 2, 2],
        }
    ).lazy()

    result = _needs_analysis(lf, n_components=2)

    assert captured["lf"] is lf
    assert captured["required_columns"] == needs_module.REQUIRED_OUTPUT_COLUMNS
    assert [name for name, _ in result] == ["feature_a", "feature_b"]
    assert result[0][1] == pytest.approx(2 / 3)
    assert result[1][1] == pytest.approx(1 / 3)


def test_make_faculty_recommendations_maps_needs_to_recommendations(monkeypatch):
    keys = list(needs_module.RECOMENDATIONS.keys())[:2]
    if not keys:
        pytest.skip("No recommendations configured")

    monkeypatch.setattr(
        needs_module,
        "_needs_analysis",
        lambda lf, n_components=3: [(key, 1.0) for key in keys],
    )

    lf = pl.DataFrame({"feature_a": [1, 2], "feature_b": [3, 4]}).lazy()

    result = make_faculty_recommendations(lf, n_components=2)

    assert result == [needs_module.RECOMENDATIONS[key] for key in keys]


def test_make_faculty_recommendations_raises_for_unknown_need(monkeypatch):
    monkeypatch.setattr(
        needs_module,
        "_needs_analysis",
        lambda lf, n_components=3: [("unknown_need", 1.0)],
    )

    lf = pl.DataFrame({"feature_a": [1, 2], "feature_b": [3, 4]}).lazy()

    with pytest.raises(ValueError, match="No se encontró una recomendación para la necesidad: unknown_need"):
        make_faculty_recommendations(lf, n_components=1)