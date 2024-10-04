use pyo3::{prelude::*, wrap_pyfunction};

mod bybit;
mod structs;

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}

#[pyfunction]
fn bybit_live_fetcher(
    py: Python,
    max_board_size: usize,
    save_time_min: i64,
    symbol: String,
    instrument: String,
    dir_path: String
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit::client::fetch(max_board_size, save_time_min, symbol, instrument, dir_path).await;
        Ok(())
    })
}
#[pymodule]
fn board_fetcher(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_live_fetcher, m)?)?;
    Ok(())
}
