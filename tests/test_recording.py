# tests/test_recording.py
"""Tests for Recording class functionality."""
import numpy as np
from unittest.mock import Mock
from intan_importer.core import Recording

def test_recording_time_computation():
    """Test that time vector is computed correctly."""
    # Create a mock Rust file
    mock_rust_file = Mock()
    mock_rust_file.header.sample_rate = 30000.0
    mock_rust_file.data_present = True
    mock_rust_file.duration.return_value = 1.0
    mock_rust_file.num_samples.return_value = 30000
    
    # Create mock data with timestamps
    mock_data = Mock()
    mock_data.timestamps = np.arange(30000, dtype=np.int32)
    mock_rust_file.data = mock_data
    
    # Create Recording
    rec = Recording(mock_rust_file)
    
    # Test time computation
    time = rec.time
    assert time is not None, "rec.time returned None"
    assert len(time) == 30000
    assert time[0] == 0.0
    assert np.isclose(time[-1], 0.9999666666666667)  # (30000-1)/30000