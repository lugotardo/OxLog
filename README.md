# Rust Logger
A flexible and lightweight logging library for Rust applications.

## Features
- Multiple log levels (Trace, Debug, Info, Warn, Error)
- File and/or stdout logging
- Configurable log level filtering
- Detailed timestamps with timezone handling
- Thread-safe design with mutex-protected file access

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
logger = { path = "path/to/logger" }
```

## Usage
Basic usage:

```rust
use logger::{Logger, LogLevel};

fn main() {
    // Create a logger that writes to both file and stdout with Info level
    let logger = Logger::new(LogLevel::Info, Some("application.log"), true);

    // Log messages with different levels
    logger.info("Application started");
    logger.warn("Configuration file not found, using defaults");
    logger.error("Failed to connect to database");

    // Debug and trace messages (only visible if log level is set accordingly)
    logger.debug("Connection pool initialized with 10 connections");
    logger.trace("Processing request #1234");
}
```

## Log Level Filtering
Messages below the configured log level will be filtered out:

```rust
// Only Warning and Error messages will be logged
let logger = Logger::new(LogLevel::Warn, Some("warnings.log"), true);

logger.info("This won't be logged");  // Filtered out
logger.warn("This will be logged");   // Logged
logger.error("This will be logged");  // Logged
```

## API Reference

### LogLevel

```rust
pub enum LogLevel {
    Trace,  // Most verbose
    Debug,
    Info,
    Warn,
    Error,  // Least verbose
}
```

### Logger

```rust
// Create a new logger
pub fn new(level: LogLevel, log_file: Option<&str>, to_stdout: bool) -> Self

// Log methods
pub fn log(&self, level: LogLevel, message: &str)
pub fn trace(&self, msg: &str)
pub fn debug(&self, msg: &str)
pub fn info(&self, msg: &str)
pub fn warn(&self, msg: &str)
pub fn error(&self, msg: &str)
```

## License
MIT License
