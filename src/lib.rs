use charms_data::{App, Data, Transaction, TxId, UtxoId, B32};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_json;

#[pymodule]
pub(crate) mod charms {

    #[pymodule_export]
    use crate::{PyApp, PyB32, PyData, PyTransaction, PyTxId, PyUtxoId};

    #[pymodule_export]
    const TOKEN: char = 't';

    #[pymodule_export]
    const NFT: char = 'n';

    #[pymodule_export]
    use crate::is_simple_transfer;

    #[pymodule_export]
    use crate::token_amounts_balanced;

    #[pymodule_export]
    use crate::nft_state_preserved;
}

#[pyclass]
#[derive(Clone)]
struct PyApp {
    inner: App,
}

#[pymethods]
impl PyApp {
    #[new]
    fn new(tag: char, identity: &PyB32, vk: &PyB32) -> Self {
       
        PyApp {
            inner: App {
                tag,
                identity: identity.inner.clone(),
                vk: vk.inner.clone(),
            },
        }
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        // Use serde_json to parse the string
        let inner: App = serde_json::from_str(&format!("\"{}\"", s))
            .map_err(|e| PyValueError::new_err(format!("invalid App string '{}': {}", s, e)))?;
        Ok(PyApp { inner })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("App({})", self.inner)
    }

    #[getter]
    fn tag(&self) -> char {
        self.inner.tag
    }

    #[getter]
    fn identity(&self) -> PyB32 {
        PyB32 {
            inner: self.inner.identity.clone(),
        }
    }

    #[getter]
    fn vk(&self) -> PyB32 {
        PyB32 {
            inner: self.inner.vk.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyTxId {
    inner: TxId,
}

#[pymethods]
impl PyTxId {
    #[new]
    fn new(bytes: &[u8]) -> PyResult<Self> {
        if bytes.len() != 32 {
            return Err(PyValueError::new_err("TxId must be 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(PyTxId { inner: TxId(arr) })
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner =
            TxId::from_str(s).map_err(|e| PyValueError::new_err(format!("invalid TxId: {}", e)))?;
        Ok(PyTxId { inner })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("TxId({})", self.inner)
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.inner.0
    }
}

#[pyclass]
#[derive(Clone)]
struct PyB32 {
    inner: B32,
}

#[pymethods]
impl PyB32 {
    #[new]
    fn new(bytes: &[u8]) -> PyResult<Self> {
        if bytes.len() != 32 {
            return Err(PyValueError::new_err("B32 must be 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(PyB32 { inner: B32(arr) })
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner =
            B32::from_str(s).map_err(|e| PyValueError::new_err(format!("invalid B32: {}", e)))?;
        Ok(PyB32 { inner })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Bytes32({})", self.inner)
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.inner.0
    }
}

#[pyclass]
#[derive(Clone)]
struct PyUtxoId {
    inner: UtxoId,
}

#[pymethods]
impl PyUtxoId {
    #[new]
    fn new(txid: &PyTxId, index: u32) -> Self {
        PyUtxoId {
            inner: UtxoId(txid.inner.clone(), index),
        }
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = UtxoId::from_str(s)
            .map_err(|e| PyValueError::new_err(format!("invalid UtxoId: {}", e)))?;
        Ok(PyUtxoId { inner })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("UtxoId({})", self.inner)
    }

    fn to_bytes(&self) -> [u8; 36] {
        self.inner.to_bytes()
    }

    #[staticmethod]
    fn from_bytes(bytes: [u8; 36]) -> Self {
        PyUtxoId {
            inner: UtxoId::from_bytes(bytes),
        }
    }

    #[getter]
    fn txid(&self) -> PyTxId {
        PyTxId {
            inner: self.inner.0.clone(),
        }
    }

    #[getter]
    fn index(&self) -> u32 {
        self.inner.1
    }
}

#[pyclass]
#[derive(Clone)]
struct PyData {
    inner: Data,
}

#[pymethods]
impl PyData {
    #[new]
    fn new() -> Self {
        PyData {
            inner: Data::empty(),
        }
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn bytes(&self) -> Vec<u8> {
        self.inner.bytes()
    }

    fn __repr__(&self) -> String {
        format!("Data({:?})", self.inner)
    }
}

#[pyclass]
struct PyTransaction {
    inner: Transaction,
}

#[pymethods]
impl PyTransaction {
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let inner: Transaction = serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(format!("invalid Transaction JSON: {}", e)))?;
        Ok(PyTransaction { inner })
    }
    
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("serialization error: {}", e)))
    }
    
    fn is_simple_transfer(&self, app: &PyApp) -> bool {
        charms_data::is_simple_transfer(&app.inner, &self.inner)
    }
    
    fn token_amounts_balanced(&self, app: &PyApp) -> bool {
        charms_data::token_amounts_balanced(&app.inner, &self.inner)
    }
    
    fn nft_state_preserved(&self, app: &PyApp) -> bool {
        charms_data::nft_state_preserved(&app.inner, &self.inner)
    }
}

#[pyfunction]
fn is_simple_transfer(app: &PyApp, tx: &PyTransaction) -> bool {
    charms_data::is_simple_transfer(&app.inner, &tx.inner)
}

#[pyfunction]
fn token_amounts_balanced(app: &PyApp, tx: &PyTransaction) -> bool {
    charms_data::token_amounts_balanced(&app.inner, &tx.inner)
}

#[pyfunction]
fn nft_state_preserved(app: &PyApp, tx: &PyTransaction) -> bool {
    charms_data::nft_state_preserved(&app.inner, &tx.inner)
}
