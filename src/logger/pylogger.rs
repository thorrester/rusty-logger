use crate::logger::rust_logger::{get_file_name, LogConfig, RustLogger};
use owo_colors::AnsiColors;
use owo_colors::OwoColorize;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};
use serde_json::{json, to_string_pretty};
use std::collections::HashMap;

#[derive(FromPyObject, Debug)]
enum PyTypes<'a> {
    CatchAll(&'a PyAny),
}

#[pyclass]
pub struct LogLevel {}

#[pymethods]
#[allow(non_snake_case)]
impl LogLevel {
    #[classattr]
    fn DEBUG() -> String {
        "DEBUG".to_string()
    }

    #[classattr]
    fn WARN() -> String {
        "WARN".to_string()
    }

    #[classattr]
    fn INFO() -> String {
        "INFO".to_string()
    }

    #[classattr]
    fn ERROR() -> String {
        "ERROR".to_string()
    }

    #[classattr]
    fn TRACE() -> String {
        "TRACE".to_string()
    }
}

pub fn parse_args(args: &PyTuple) -> Option<Vec<String>> {
    let args: Option<Vec<String>> = if args.is_empty() {
        None
    } else {
        Some(
            args.iter()
                .map(|x| match x.extract::<PyTypes>() {
                    Ok(PyTypes::CatchAll(c)) => c.to_string(),
                    Err(e) => {
                        println!("Error: {}", e);
                        "".to_string()
                    }
                })
                .collect::<Vec<String>>(),
        )
    };

    args
}

#[pyclass(name = "Logger", subclass)]
pub struct PyLogger {
    logger: RustLogger,

    #[pyo3(get, set)]
    pub config: LogConfig,
}

#[pymethods]
#[allow(unused_variables)]
impl PyLogger {
    /// Create a new logger
    ///
    /// # Arguments
    /// * `config` - The log config to use
    ///
    /// # Returns
    /// A new logger
    #[classmethod]
    pub fn get_logger(cls: &PyType, name: Option<String>, config: Option<LogConfig>) -> PyLogger {
        let mut log_config = config.unwrap_or_else(|| {
            // get default
            LogConfig::new(
                None, None, None, None, None, None, None, None, None, None, None,
            )
        });

        // in case where user provides log config and name
        // check if user provided name in log config first
        if log_config.name.is_none() {
            log_config.name = name.map(|val| get_file_name(&val));
        }

        let logger = RustLogger::new(&log_config);

        PyLogger {
            logger,
            config: log_config,
        }
    }

    /// Set the log level for the logger
    ///
    /// # Arguments
    /// * `level` - The log level to set
    ///
    pub fn set_level(&mut self, level: String) {
        let mut config = self.config.clone();
        config.log_level(level);
        self.logger.reload_level(&config.level).unwrap()
    }

    fn check_format_kwargs(&self, message: &str, kwargs: Option<HashMap<&str, &str>>) -> String {
        if self.config.file_config.is_some() {
            return message.to_string();
        };

        if kwargs.is_some() {
            let kwargs = kwargs.unwrap();
            let color = kwargs.get("color").expect("No color provided");

            let ansi_color = match *color {
                "red" => AnsiColors::Red,
                "green" => AnsiColors::Green,
                "yellow" => AnsiColors::Yellow,
                "blue" => AnsiColors::Blue,
                "magenta" => AnsiColors::Magenta,
                "cyan" => AnsiColors::Cyan,
                "white" => AnsiColors::White,
                "black" => AnsiColors::Black,
                _ => AnsiColors::Black,
            };

            message.color(ansi_color).to_string()
        } else {
            message.to_string()
        }
    }

    /// Log at INFO level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `kwargs` - Optional formatting kwargs
    ///
    #[pyo3(signature = (message, *args, **kwargs))]
    pub fn info(&self, message: &str, args: &PyTuple, kwargs: Option<HashMap<&str, &str>>) {
        let args = parse_args(args);
        let message = self.check_format_kwargs(message, kwargs);
        self.logger.info(&message, args, &self.config);
    }

    /// Log at DEBUG level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `kwargs` - Optional formatting kwargs
    ///
    #[pyo3(signature = (message, *args, **kwargs))]
    pub fn debug(&self, message: &str, args: &PyTuple, kwargs: Option<HashMap<&str, &str>>) {
        let args = parse_args(args);
        let message = self.check_format_kwargs(message, kwargs);
        self.logger.debug(&message, args, &self.config);
    }

    /// Log at WARN level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `kwargs` - Optional formatting kwargs
    ///
    #[pyo3(signature = (message, *args, **kwargs))]
    pub fn warning(&self, message: &str, args: &PyTuple, kwargs: Option<HashMap<&str, &str>>) {
        let args = parse_args(args);
        let message = self.check_format_kwargs(message, kwargs);
        self.logger.warning(&message, args, &self.config);
    }

    /// Log at ERROR level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `kwargs` - Optional formatting kwargs
    ///
    #[pyo3(signature = (message, *args, **kwargs))]
    pub fn error(&self, message: &str, args: &PyTuple, kwargs: Option<HashMap<&str, &str>>) {
        let args = parse_args(args);
        let message = self.check_format_kwargs(message, kwargs);
        self.logger.error(&message, args, &self.config);
    }

    /// Log at TRACE level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `kwargs` - Optional formatting kwargs
    ///
    #[pyo3(signature = (message, *args, **kwargs))]
    pub fn trace(&self, message: &str, args: &PyTuple, kwargs: Option<HashMap<&str, &str>>) {
        let args = parse_args(args);
        let message = self.check_format_kwargs(message, kwargs);
        self.logger.trace(&message, args, &self.config);
    }

    /// String magic method for PyLogger class
    pub fn __str__(&self) -> PyResult<String> {
        let json = json!({
            "type": "Logger",
            "name": self.config.name,
            "level": self.config.app_env,
            "config": self.config,
        });

        Ok(to_string_pretty(&json).unwrap())
    }
}
