

Fast Python bindings for reading Intan RHS files, powered by Rust for high performance.

> **Note**: Version 0.2.0 introduces a breaking change - analog data arrays now return float64 values in physical units instead of raw int32 ADC values. See [MIGRATION_GUIDE_v0.2.0.md](MIGRATION_GUIDE_v0.2.0.md) for details.

## Quick Start

```python
import intan_importer

# Load a single RHS file or a directory of files
rec = intan_importer.load("data.rhs")

# Access the data
time = rec.data.time               # Time vector in seconds
data = rec.data.amplifier_data     # Neural data in microvolts
```

## Data Structure

```
Recording object (rec)
│
├── .duration                → total duration in seconds
├── .num_samples             → total number of samples
├── .sample_rate             → sampling rate in Hz
├── .num_channels            → number of amplifier channels
├── .data_present            → bool, whether data exists
├── .source_files            → list of source files (if multiple)
│
├── .header
│   ├── .sample_rate         → sampling rate in Hz
│   ├── .notch_filter_frequency → 50, 60, or None
│   ├── .reference_channel   → reference channel name
│   ├── .note1, .note2, .note3 → user notes
│   ├── .amplifier_channels  → list of channel info
│   └── .board_adc_channels  → list of ADC channel info
│
├── .data (if present)
│   ├── .time                → time vector in seconds (float64)
│   ├── .timestamps          → sample numbers (int32)
│   ├── .amplifier_data      → neural data (µV, float64)
│   ├── .board_adc_data      → auxiliary inputs (V, float64)
│   ├── .board_dig_in_data   → digital inputs (0 or 1)
│   ├── .stim_data           → stimulation current (µA)
│   └── ... (other optional data types)
│
└── Methods:
    ├── .get_channel_data(channel, start_time, end_time)
    ├── .get_time_slice(start_time, end_time) 
    └── .summary()
```

## Key Features

### Time Vectors
Both time representations are available in the data object:
```python
time = rec.data.time          # Time in seconds (computed from timestamps)
timestamps = rec.data.timestamps  # Raw sample numbers

# Convert between them
time_manual = timestamps / rec.sample_rate  # Same as rec.data.time
```

### Channel Access
Access channels by index or name:
```python
# By index
data = rec.get_channel_data(0)

# By channel name
data = rec.get_channel_data("CA1")

# With time window (in seconds)
data = rec.get_channel_data("CA1", start_time=10.0, end_time=20.0)
```

### Channel Information
```python
for i, ch in enumerate(rec.header.amplifier_channels):
    print(f"Channel {i}: {ch.custom_channel_name} ({ch.electrode_impedance_magnitude:.0f} Ω)")
```


## Data Types

- **Time**: Seconds (float64)
- **Timestamps**: Sample numbers (int32)
- **Amplifier data**: Microvolts (µV)
- **ADC data**: Volts (V)
- **Digital data**: Binary (0 or 1)
- **Stimulation data**: Microamps (µA)

