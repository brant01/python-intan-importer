// src/lib.rs
//! Python bindings for the intan_importer crate.
//! 
//! This module provides a Python-accessible interface to load Intan RHS files
//! using the high-performance Rust implementation.

use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;
use numpy::{PyArray1, PyArray2, IntoPyArray};
use intan_importer as intan;
use std::sync::Arc;

/// Python representation of channel information
#[pyclass]
#[derive(Clone)]
struct ChannelInfo {
    /// Name of the port (e.g., "Port A")
    #[pyo3(get)]
    port_name: String,
    
    /// Default channel name assigned by the system
    #[pyo3(get)]
    native_channel_name: String,
    
    /// User-defined custom name for the channel
    #[pyo3(get)]
    custom_channel_name: String,
    
    /// Original order in the native system
    #[pyo3(get)]
    native_order: i32,
    
    /// Channel on the chip
    #[pyo3(get)]
    chip_channel: i32,
    
    /// Measured electrode impedance magnitude (Ω)
    #[pyo3(get)]
    electrode_impedance_magnitude: f32,
    
    /// Measured electrode impedance phase (radians)
    #[pyo3(get)]
    electrode_impedance_phase: f32,
}

#[pymethods]
impl ChannelInfo {
    fn __repr__(&self) -> String {
        format!(
            "ChannelInfo(name='{}', chip_channel={}, impedance={:.0}Ω)",
            self.custom_channel_name,
            self.chip_channel,
            self.electrode_impedance_magnitude
        )
    }
}

/// Python representation of an RHS file header.
/// 
/// Contains all metadata and configuration information from the recording file.
#[pyclass]
struct RhsHeader {
    // Store the Rust header in an Arc for thread safety
    inner: Arc<intan::RhsHeader>,
}

#[pymethods]
impl RhsHeader {
    /// Sample rate in Hz
    #[getter]
    fn sample_rate(&self) -> f32 {
        self.inner.sample_rate
    }
    
    /// Number of samples per data block (always 128 for RHS)
    #[getter]
    fn num_samples_per_data_block(&self) -> i32 {
        self.inner.num_samples_per_data_block
    }
    
    /// Whether DC amplifier data was saved
    #[getter]
    fn dc_amplifier_data_saved(&self) -> bool {
        self.inner.dc_amplifier_data_saved
    }
    
    /// Notch filter frequency (50, 60, or None)
    #[getter]
    fn notch_filter_frequency(&self) -> Option<i32> {
        self.inner.notch_filter_frequency
    }
    
    /// Reference channel name
    #[getter]
    fn reference_channel(&self) -> &str {
        &self.inner.reference_channel
    }
    
    /// Number of amplifier channels
    #[getter]
    fn num_amplifier_channels(&self) -> usize {
        self.inner.amplifier_channels.len()
    }
    
    /// Number of board ADC channels
    #[getter]
    fn num_board_adc_channels(&self) -> usize {
        self.inner.board_adc_channels.len()
    }
    
    /// Number of board DAC channels
    #[getter]
    fn num_board_dac_channels(&self) -> usize {
        self.inner.board_dac_channels.len()
    }
    
    /// Number of board digital input channels
    #[getter]
    fn num_board_dig_in_channels(&self) -> usize {
        self.inner.board_dig_in_channels.len()
    }
    
    /// Number of board digital output channels
    #[getter]
    fn num_board_dig_out_channels(&self) -> usize {
        self.inner.board_dig_out_channels.len()
    }
    
    /// Get list of amplifier channels
    #[getter]
    fn amplifier_channels(&self) -> Vec<ChannelInfo> {
        self.inner.amplifier_channels.iter().map(|ch| ChannelInfo {
            port_name: ch.port_name.clone(),
            native_channel_name: ch.native_channel_name.clone(),
            custom_channel_name: ch.custom_channel_name.clone(),
            native_order: ch.native_order,
            chip_channel: ch.chip_channel,
            electrode_impedance_magnitude: ch.electrode_impedance_magnitude,
            electrode_impedance_phase: ch.electrode_impedance_phase,
        }).collect()
    }
    
    /// Get list of board ADC channels
    #[getter]
    fn board_adc_channels(&self) -> Vec<ChannelInfo> {
        self.inner.board_adc_channels.iter().map(|ch| ChannelInfo {
            port_name: ch.port_name.clone(),
            native_channel_name: ch.native_channel_name.clone(),
            custom_channel_name: ch.custom_channel_name.clone(),
            native_order: ch.native_order,
            chip_channel: ch.chip_channel,
            electrode_impedance_magnitude: ch.electrode_impedance_magnitude,
            electrode_impedance_phase: ch.electrode_impedance_phase,
        }).collect()
    }
    
    /// Get the first note
    #[getter]
    fn note1(&self) -> &str {
        &self.inner.notes.note1
    }
    
    /// Get the second note
    #[getter]
    fn note2(&self) -> &str {
        &self.inner.notes.note2
    }
    
    /// Get the third note
    #[getter]
    fn note3(&self) -> &str {
        &self.inner.notes.note3
    }
    
    fn __repr__(&self) -> String {
        format!(
            "RhsHeader(sample_rate={}, amplifier_channels={}, adc_channels={})",
            self.inner.sample_rate,
            self.inner.amplifier_channels.len(),
            self.inner.board_adc_channels.len()
        )
    }
}

/// Python representation of RHS data.
/// 
/// Contains the actual recorded signals from all enabled channels.
#[pyclass]
struct RhsData {
    inner: Arc<intan::RhsData>,
    sample_rate: f32,  // Store sample rate for convenience
}

#[pymethods]
impl RhsData {
    /// Get timestamps as a NumPy array (in samples, divide by sample_rate for seconds)
    #[getter]
    fn timestamps<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<i32>> {
        self.inner.timestamps.clone().into_pyarray_bound(py)
    }
    
    /// Get amplifier data as a NumPy array (μV, shape: [channels, samples])
    #[getter]
    fn amplifier_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<f64>>> {
        self.inner.amplifier_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get DC amplifier data as a NumPy array (V, shape: [channels, samples])
    #[getter]
    fn dc_amplifier_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<f64>>> {
        self.inner.dc_amplifier_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get stimulation data as a NumPy array (μA, shape: [channels, samples])
    #[getter]
    fn stim_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<i32>>> {
        self.inner.stim_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get compliance limit data as a NumPy array (bool, shape: [channels, samples])
    #[getter]
    fn compliance_limit_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<bool>>> {
        self.inner.compliance_limit_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get charge recovery data as a NumPy array (bool, shape: [channels, samples])
    #[getter]
    fn charge_recovery_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<bool>>> {
        self.inner.charge_recovery_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get amplifier settle data as a NumPy array (bool, shape: [channels, samples])
    #[getter]
    fn amp_settle_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<bool>>> {
        self.inner.amp_settle_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get board ADC data as a NumPy array (V, shape: [channels, samples])
    #[getter]
    fn board_adc_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<f64>>> {
        self.inner.board_adc_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get board DAC data as a NumPy array (V, shape: [channels, samples])
    #[getter]
    fn board_dac_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<f64>>> {
        self.inner.board_dac_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get board digital input data as a NumPy array (0 or 1, shape: [channels, samples])
    #[getter]
    fn board_dig_in_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<i32>>> {
        self.inner.board_dig_in_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    /// Get board digital output data as a NumPy array (0 or 1, shape: [channels, samples])
    #[getter]
    fn board_dig_out_data<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray2<i32>>> {
        self.inner.board_dig_out_data.as_ref()
            .map(|data| data.clone().into_pyarray_bound(py))
    }
    
    fn __repr__(&self) -> String {
        let num_samples = self.inner.timestamps.len();
        let duration = num_samples as f32 / self.sample_rate;
        format!("RhsData(samples={}, duration={:.2}s)", num_samples, duration)
    }
}

/// Python representation of an RHS file.
/// 
/// This class mirrors the Rust RhsFile structure, containing header
/// information and optionally the recorded data.
#[pyclass]
struct RhsFile {
    /// Store the original Rust file
    inner: Arc<intan::RhsFile>,
}

#[pymethods]
impl RhsFile {
    /// Header information
    #[getter]
    fn header(&self) -> PyResult<RhsHeader> {
        Ok(RhsHeader {
            inner: Arc::new(self.inner.header.clone()),
        })
    }
    
    /// Whether data is present (vs just header)
    #[getter]
    fn data_present(&self) -> bool {
        self.inner.data_present
    }
    
    /// List of source files if this was created by combining multiple files
    #[getter]
    fn source_files(&self) -> Option<Vec<String>> {
        self.inner.source_files.clone()
    }
    
    /// The data (if present)
    #[getter]
    fn data(&self) -> Option<RhsData> {
        self.inner.data.as_ref().map(|data| RhsData {
            inner: Arc::new(data.clone()),
            sample_rate: self.inner.header.sample_rate,
        })
    }
    
    /// Returns the duration of the recording in seconds
    fn duration(&self) -> f32 {
        self.inner.duration()
    }
    
    /// Returns the number of samples in the recording
    fn num_samples(&self) -> usize {
        self.inner.num_samples()
    }
    
    /// String representation of the RhsFile
    fn __repr__(&self) -> String {
        format!(
            "RhsFile(sample_rate={}, channels={}, duration={:.2}s, data_present={})",
            self.inner.header.sample_rate,
            self.inner.header.amplifier_channels.len(),
            self.duration(),
            self.inner.data_present
        )
    }
}

/// Load an Intan RHS file or directory containing RHS files.
/// 
/// Args:
///     path (str): Path to a single .rhs file or a directory containing .rhs files
/// 
/// Returns:
///     RhsFile: An object containing the header and data from the file(s)
/// 
/// Raises:
///     IOError: If the file cannot be read or is not a valid RHS file
#[pyfunction]
fn load(_py: Python, path: &str) -> PyResult<RhsFile> {
    match intan::load(path) {
        Ok(rust_file) => {
            Ok(RhsFile {
                inner: Arc::new(rust_file),
            })
        },
        Err(e) => {
            Err(PyIOError::new_err(format!("Failed to load file: {}", e)))
        }
    }
}

/// Python module definition
#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add our load function to the module
    m.add_function(wrap_pyfunction!(load, m)?)?;
    
    // Add the classes
    m.add_class::<RhsFile>()?;
    m.add_class::<RhsHeader>()?;
    m.add_class::<RhsData>()?;
    m.add_class::<ChannelInfo>()?;
    
    // Add module metadata
    m.add("__version__", "0.2.0")?;
    m.add("__doc__", "Low-level bindings for intan_importer")?;
    
    Ok(())
}