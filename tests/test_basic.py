

# test_basic.py
import intan_importer
import numpy as np

# Test loading a single file
print("Testing intan_importer...")

# Replace with your actual test file path
test_file = "test_data/20231220_IC_test/Test_A/20231220_IC_Test_16c_231220_153815"
#test_file = "test_data/20231220_IC_test/Test_B/20231220_IC_TEST_B_231220_160314/20231220_IC_TEST_B_231220_160314.rhs"


try:
    # Load the file
    recording = intan_importer.load(test_file)
    
    print(f"\nFile loaded successfully!")
    print(f"Recording: {recording}")
    print(f"Header: {recording.header}")
    print(f"Data present: {recording.data_present}")
    print(f"Duration: {recording.duration():.2f} seconds")
    print(f"Number of samples: {recording.num_samples()}")
    
    # Check header details
    header = recording.header
    print(f"\nHeader details:")
    print(f"  Sample rate: {header.sample_rate} Hz")
    print(f"  Amplifier channels: {header.num_amplifier_channels}")
    print(f"  ADC channels: {header.num_board_adc_channels}")
    print(f"  Digital inputs: {header.num_board_dig_in_channels}")
    print(f"  DC amplifier data saved: {header.dc_amplifier_data_saved}")
    print(f"  Notch filter: {header.notch_filter_frequency} Hz" if header.notch_filter_frequency else "  Notch filter: None")
    
    # Check source files if multiple were combined
    if recording.source_files:
        print(f"\nCombined from {len(recording.source_files)} files:")
        for i, source in enumerate(recording.source_files):
            print(f"  {i+1}: {source}")
    
    # Check data if present
    if recording.data_present and recording.data:
        data = recording.data
        print(f"\nData: {data}")
        
        # Check timestamps
        timestamps = data.timestamps
        print(f"\nTimestamps:")
        print(f"  Shape: {timestamps.shape}")
        print(f"  First 5: {timestamps[:5]}")
        print(f"  Last 5: {timestamps[-5:]}")
        print(f"  Time of last sample: {timestamps[-1] / header.sample_rate:.3f} seconds")
        
        # Check amplifier data if present
        if data.amplifier_data is not None:
            amp_data = data.amplifier_data
            print(f"\nAmplifier data:")
            print(f"  Shape: {amp_data.shape} (channels × samples)")
            print(f"  Data type: {amp_data.dtype}")
            print(f"  Units: microvolts")
            
            # Show example of how to convert timestamp to seconds
            print(f"\nExample: Sample 30000 is at t = {timestamps[30000] / header.sample_rate:.3f} seconds")
            
        # Check other data types if present
        if data.board_adc_data is not None:
            print(f"\nBoard ADC data: shape {data.board_adc_data.shape}")
            
        if data.dc_amplifier_data is not None:
            print(f"\nDC amplifier data: shape {data.dc_amplifier_data.shape}")
            
except Exception as e:
    print(f"Error: {e}")
    import traceback
    traceback.print_exc()