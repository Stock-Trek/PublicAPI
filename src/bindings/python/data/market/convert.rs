#[cfg(feature = "python")]
use {
    pyo3::{
        prelude::*,
        types::{PyDict, PyList, PyModule},
        IntoPyObject,
    },
    rust_decimal::Decimal,
    std::{collections::HashMap, sync::OnceLock},
};

#[cfg(feature = "python")]
static DECIMAL_CLS: OnceLock<Py<PyAny>> = OnceLock::new();

#[cfg(feature = "python")]
pub fn hashmap_to_dict<K, V>(py: Python<'_>, map: &HashMap<K, V>) -> Py<PyDict>
where
    K: for<'py> IntoPyObject<'py> + Clone + Sync,
    V: for<'py> IntoPyObject<'py> + Clone + Sync,
{
    let dict = PyDict::new(py);
    for (key, value) in map {
        dict.set_item(key.clone(), value.clone()).unwrap();
    }
    dict.into()
}

#[cfg(feature = "python")]
pub fn vec_to_list<V>(py: Python<'_>, vec: &Vec<V>) -> Py<PyList>
where
    V: for<'py> IntoPyObject<'py> + Clone + Sync,
{
    let list = PyList::empty(py);
    for value in vec {
        list.add(value.clone()).unwrap();
    }
    list.into()
}

#[cfg(feature = "python")]
pub fn rust_decimal_to_py(py: Python<'_>, val: Decimal) -> PyResult<Py<PyAny>> {
    let s = val.to_string();
    let cls = get_decimal_cls(py).bind(py);
    let obj = cls.call1((s,))?;
    Ok(obj.unbind())
}

#[cfg(feature = "python")]
fn get_decimal_cls(py: Python<'_>) -> &Py<PyAny> {
    DECIMAL_CLS.get_or_init(|| {
        let module = PyModule::import(py, "decimal").unwrap();
        module.getattr("Decimal").unwrap().unbind()
    })
}
