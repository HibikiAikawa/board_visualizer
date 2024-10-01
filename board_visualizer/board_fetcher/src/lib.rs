use pyo3::{prelude::*, wrap_pyfunction};

mod coincheck;
mod structs;

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}

// TODO 引数はどうやって入れる？
#[pyfunction]
fn run_coincheck(py: Python, max_board_size: usize) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        coincheck::client::run(max_board_size).await;
        Ok(())
    })
}
#[pymodule]
fn board_fetcher(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(run_coincheck, m)?)?;
    Ok(())
}
