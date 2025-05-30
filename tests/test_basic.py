# tests/test_basic.py
"""Basic functionality tests for intan_importer."""
import pytest
import intan_importer
from pathlib import Path

def test_import():
    """Test that the module can be imported."""
    assert hasattr(intan_importer, 'load')
    assert hasattr(intan_importer, '__version__')
    assert hasattr(intan_importer, 'Recording')

def test_version():
    """Test version is accessible."""
    assert isinstance(intan_importer.__version__, str)
    assert intan_importer.__version__ == "0.1.0"

# Since we don't have test data files in the repo, we can only test the structure
def test_recording_class_exists():
    """Test that Recording class is available."""
    assert intan_importer.Recording is not None

# If you want to test with actual files during development, you can mark tests
# to skip them in CI or when files aren't available
@pytest.mark.skipif(
    not Path("test_data/sample.rhs").exists(),
    reason="Test data not available"
)
def test_load_single_file():
    """Test loading a single RHS file."""
    rec = intan_importer.load("test_data/sample.rhs")
    assert rec.data_present
    assert rec.duration > 0
    assert rec.num_samples > 0