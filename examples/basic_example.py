# examples/basic_usage.py
"""Example of loading and accessing Intan RHS data."""
import intan_importer
import numpy as np
import sys

if len(sys.argv) < 2:
    print("Usage: python basic_usage.py <path_to_rhs_file_or_directory>")
    sys.exit(1)

# Load the data
path = sys.argv[1]
recording = intan_importer.load(path)

print(f"Loaded: {recording}")
print(f"Duration: {recording.duration():.2f} seconds")
print(f"Channels: {recording.header.num_amplifier_channels}")

# Access the data
if recording.data and recording.data.amplifier_data is not None:
    data = recording.data.amplifier_data
    timestamps = recording.data.timestamps
    sample_rate = recording.header.sample_rate
    
    # Example: Extract 1 second of data from channel 0
    start_sample = 0
    end_sample = int(sample_rate)  # 1 second worth
    
    channel_data = data[0, start_sample:end_sample]
    time_seconds = timestamps[start_sample:end_sample] / sample_rate
    
    print(f"\nFirst second of data from channel 0:")
    print(f"  Samples: {len(channel_data)}")
    print(f"  Mean: {np.mean(channel_data):.2f} µV")
    print(f"  Std: {np.std(channel_data):.2f} µV")