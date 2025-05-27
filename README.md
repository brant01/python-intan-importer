


# intan-importer

Fast Python bindings for reading Intan RHS files, powered by Rust for high performance.

## Installation

```bash
pip install intan-importer
```

## Quick Start

```python
import intan_importer

# Load a single RHS file
recording = intan_importer.load("path/to/file.rhs")

# Or load all RHS files from a directory
recording = intan_importer.load("path/to/directory/")

# Access metadata
print(f"Sample rate: {recording.header.sample_rate} Hz")
print(f"Duration: {recording.duration()} seconds")

# Access data as NumPy arrays
if recording.data:
    timestamps = recording.data.timestamps  # Sample numbers
    amplifier_data = recording.data.amplifier_data  # Shape: (channels, samples)
    
    # Convert timestamps to seconds
    time_seconds = timestamps / recording.header.sample_rate
```

## Data Units

- **Timestamps**: Sample numbers (divide by sample_rate for seconds)
- **Amplifier data**: Microvolts (µV)
- **DC amplifier data**: Volts (V)
- **ADC/DAC data**: Volts (V)
- **Digital data**: 0 or 1

## License

Licensed under MIT.
