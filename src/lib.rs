use charms_data::{App, Data, Transaction, TxId, UtxoId, B32, util};
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

impl PyData {
    pub(crate) fn from_py(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(py_data) = obj.extract::<PyData>() {
            return Ok(py_data);
        }
        
        if let Ok(json_str) = obj.extract::<String>() {
            let value: serde_json::Value = serde_json::from_str(&json_str)
                .map_err(|e| PyValueError::new_err(format!("invalid JSON: {}", e)))?;
            return Ok(PyData {
                inner: Data::from(&value),
            });
        }
        
        if let Ok(bytes) = obj.extract::<Vec<u8>>() {
            match util::read::<Data, &[u8]>(bytes.as_slice()) {
                Ok(data) => return Ok(PyData { inner: data }),
                Err(_) => {
                    // If not valid CBOR, try to interpret as JSON string
                    if let Ok(json_str) = String::from_utf8(bytes) {
                        let value: serde_json::Value = serde_json::from_str(&json_str)
                            .map_err(|e| PyValueError::new_err(format!("invalid JSON: {}", e)))?;
                        return Ok(PyData {
                            inner: Data::from(&value),
                        });
                    }
                }
            }
        }
        
        if let Ok(py_int) = obj.extract::<i64>() {
            let value = serde_json::Value::from(py_int);
            return Ok(PyData {
                inner: Data::from(&value),
            });
        }
        if let Ok(py_float) = obj.extract::<f64>() {
            let value = serde_json::Value::from(py_float);
            return Ok(PyData {
                inner: Data::from(&value),
            });
        }
        if let Ok(py_bool) = obj.extract::<bool>() {
            let value = serde_json::Value::from(py_bool);
            return Ok(PyData {
                inner: Data::from(&value),
            });
        }
        if obj.is_none() {
            let value = serde_json::Value::Null;
            return Ok(PyData {
                inner: Data::from(&value),
            });
        }
        
        // For lists, dicts, and other objects, use Python's json module
        // to convert to JSON string, then parse with serde_json
        let json_module = PyModule::import(obj.py(), "json")?;
        let json_dumps = json_module.getattr("dumps")?;
        let json_str: String = json_dumps.call1((obj,))?.extract()?;
        let value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PyValueError::new_err(format!("cannot convert to JSON: {}", e)))?;
        Ok(PyData {
            inner: Data::from(&value),
        })
    }
}

#[pymethods]
impl PyData {
    #[new]
    #[pyo3(signature = (obj = None))]
    fn new(obj: Option<&Bound<PyAny>>) -> PyResult<Self> {
        match obj {
            Some(obj) => Self::from_py(obj),
            None => Ok(PyData {
                inner: Data::empty(),
            }),
        }
    }

    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(format!("invalid JSON: {}", e)))?;
        Ok(PyData {
            inner: Data::from(&value),
        })
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn bytes(&self) -> Vec<u8> {
        self.inner.bytes()
    }

    fn to_json(&self) -> PyResult<String> {
        // Deserialize Data to serde_json::Value, then serialize to JSON string
        let value: serde_json::Value = self.inner
            .value()
            .map_err(|e| PyValueError::new_err(format!("deserialization error: {}", e)))?;
        serde_json::to_string(&value)
            .map_err(|e| PyValueError::new_err(format!("serialization error: {}", e)))
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
