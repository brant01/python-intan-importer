
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
```

## Data Structure

The loaded `RhsFile` object contains:
- `header`: Metadata and configuration information
- `data`: The actual recorded signals (if present)
- `data_present`: Boolean indicating if data is present
- `source_files`: List of source files (if loaded from directory)

### Accessing the Header

```python
header = recording.header

# Basic information
header.sample_rate                    # float, Hz (e.g., 30000.0)
header.num_samples_per_data_block     # int, always 128 for RHS
header.reference_channel              # str, name of reference channel

# Channel counts
header.num_amplifier_channels         # int, number of neural channels
header.num_board_adc_channels         # int, number of ADC channels
header.num_board_dac_channels         # int, number of DAC channels
header.num_board_dig_in_channels      # int, number of digital inputs
header.num_board_dig_out_channels     # int, number of digital outputs

# Recording settings
header.dc_amplifier_data_saved        # bool, whether DC data was saved
header.notch_filter_frequency         # int or None, 50/60 Hz or None

# Notes (user annotations)
header.note1                          # str
header.note2                          # str
header.note3                          # str

# Channel information (list of ChannelInfo objects)
header.amplifier_channels             # List[ChannelInfo]
header.board_adc_channels             # List[ChannelInfo]
```

### Channel Information

Each channel in the lists has these attributes:

```python
channel = header.amplifier_channels[0]

channel.port_name                     # str, e.g., "Port A"
channel.native_channel_name           # str, e.g., "A-000"
channel.custom_channel_name           # str, user-defined name
channel.native_order                  # int, original order
channel.chip_channel                  # int, channel on chip
channel.electrode_impedance_magnitude # float, Ohms
channel.electrode_impedance_phase     # float, radians
```

### Accessing the Data

All data arrays are NumPy arrays with shape `(channels, samples)`:

```python
data = recording.data

# Core data arrays
data.timestamps         # ndarray[int32], shape: (samples,)
                       # Sample numbers, NOT seconds
                       # Convert to seconds: timestamps / header.sample_rate

data.amplifier_data    # ndarray[int32] or None, shape: (channels, samples)
                       # Neural signals in microvolts (µV)

data.board_adc_data    # ndarray[int32] or None, shape: (channels, samples)
                       # Auxiliary analog inputs in volts (V)
```

#### Optional Data Arrays (may be None)

```python
# DC-coupled amplifier data
data.dc_amplifier_data  # ndarray[int32] or None, shape: (channels, samples)
                        # DC amplifier signals in volts (V)

# Stimulation data
data.stim_data          # ndarray[int32] or None, shape: (channels, samples)
                        # Stimulation current in microamps (µA)

# Stimulation status arrays (boolean)
data.compliance_limit_data  # ndarray[bool] or None, shape: (channels, samples)
                           # True when compliance limit reached

data.charge_recovery_data   # ndarray[bool] or None, shape: (channels, samples)
                           # True when charge recovery active

data.amp_settle_data        # ndarray[bool] or None, shape: (channels, samples)
                           # True when amplifier settling

# Digital I/O
data.board_dig_in_data  # ndarray[int32] or None, shape: (channels, samples)
                        # Digital inputs, 0 or 1

data.board_dig_out_data # ndarray[int32] or None, shape: (channels, samples)
                        # Digital outputs, 0 or 1

# Analog outputs
data.board_dac_data     # ndarray[int32] or None, shape: (channels, samples)
                        # DAC outputs in volts (V)
```

## Common Usage Examples

### Get Recording Information

```python
# Basic info
duration_seconds = recording.duration()
num_samples = recording.num_samples()
sample_rate = recording.header.sample_rate

print(f"Duration: {duration_seconds:.2f} seconds")
print(f"Sampling rate: {sample_rate} Hz")
print(f"Number of channels: {recording.header.num_amplifier_channels}")
```

### Extract Neural Data

```python
if recording.data and recording.data.amplifier_data is not None:
    # Get all neural data
    neural_data = recording.data.amplifier_data  # shape: (channels, samples)
    
    # Get single channel
    channel_0 = neural_data[0, :]  # 1D array of samples
    
    # Get time vector in seconds
    timestamps = recording.data.timestamps
    time_seconds = timestamps / recording.header.sample_rate
    
    # Get specific time range (e.g., seconds 10-20)
    start_sample = int(10 * sample_rate)
    end_sample = int(20 * sample_rate)
    data_segment = neural_data[:, start_sample:end_sample]
```

### Find Channels by Name

```python
# Find a channel by custom name
target_name = "CA1"
for i, channel in enumerate(recording.header.amplifier_channels):
    if channel.custom_channel_name == target_name:
        channel_index = i
        channel_data = recording.data.amplifier_data[i, :]
        break
```

### Work with Digital Inputs

```python
if recording.data and recording.data.board_dig_in_data is not None:
    digital_data = recording.data.board_dig_in_data
    
    # Find rising edges on digital channel 0
    channel_0 = digital_data[0, :]
    rising_edges = np.where(np.diff(channel_0) > 0)[0] + 1
    
    # Convert to timestamps in seconds
    edge_times = recording.data.timestamps[rising_edges] / sample_rate
```

### Check for Stimulation

```python
if recording.data and recording.data.stim_data is not None:
    stim_current = recording.data.stim_data  # in microamps
    
    # Find when stimulation was active (non-zero current)
    stim_active = np.any(stim_current != 0, axis=0)
    stim_times = recording.data.timestamps[stim_active] / sample_rate
    
    # Check compliance limits
    if recording.data.compliance_limit_data is not None:
        compliance_events = np.any(recording.data.compliance_limit_data, axis=0)
        print(f"Compliance limit reached in {np.sum(compliance_events)} samples")
```

## Data Type Summary

| Data | Type | Shape | Units | Range |
|------|------|-------|-------|-------|
| timestamps | int32 | (samples,) | sample number | 0 to N-1 |
| amplifier_data | int32 | (channels, samples) | microvolts | ±6389.76 mV |
| dc_amplifier_data | int32 | (channels, samples) | volts | ±10.24 V |
| stim_data | int32 | (channels, samples) | microamps | varies |
| board_adc_data | int32 | (channels, samples) | volts | ±10.24 V |
| board_dac_data | int32 | (channels, samples) | volts | ±10.24 V |
| board_dig_in_data | int32 | (channels, samples) | binary | 0 or 1 |
| board_dig_out_data | int32 | (channels, samples) | binary | 0 or 1 |
| compliance_limit_data | bool | (channels, samples) | boolean | True/False |
| charge_recovery_data | bool | (channels, samples) | boolean | True/False |
| amp_settle_data | bool | (channels, samples) | boolean | True/False |

## Notes

- All data is returned as NumPy arrays for easy integration with scientific Python tools
- Timestamps are sample numbers (not seconds) to preserve precision
- Missing data types return `None` rather than empty arrays
- Memory usage: ~4 bytes per sample per channel for int32 data

## License

Licensed under MIT.
