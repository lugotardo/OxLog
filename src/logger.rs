use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, Arc};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info  => "INFO",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

pub struct Logger {
    level: LogLevel,
    file: Option<Arc<Mutex<File>>>,
    to_stdout: bool,
}

impl Logger {
    pub fn new(level: LogLevel, log_file: Option<&str>, to_stdout: bool) -> Self {
        let file = log_file.map(|path| {
            Arc::new(Mutex::new(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Unable to open log file"),
            ))
        });
        Logger { level, file, to_stdout }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if (level as u8) < (self.level as u8) {
            return;
        }
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();
        let total_secs = secs;
        
        #[cfg(target_os = "linux")]
        let timezone_offset_seconds = {
            if let Ok(tz_str) = read_to_string("/etc/timezone") {
            if tz_str.trim() == "America/Sao_Paulo" {
                -3 * 3600 
            } else {
                0
            }
            } else {
            0
            }
        };
        
        #[cfg(not(target_os = "linux"))]
        let timezone_offset_seconds = -3 * 3600; 
        let time_with_offset = (total_secs as i64 + timezone_offset_seconds) as u64;
        let secs = time_with_offset % 60;
        let mins = (time_with_offset / 60) % 60;
        let hours = (time_with_offset / 3600) % 24;
        let days_since_epoch = time_with_offset / 86400;
        let mut year = 1970;
        let mut remaining_days = days_since_epoch;
        while remaining_days > 0 {
            let days_in_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            366
            } else {
            365
            };
            
            if remaining_days >= days_in_year {
            remaining_days -= days_in_year;
            year += 1;
            } else {
            break;
            }
        }
        let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let month_days = [31, if is_leap_year { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 0;
        let mut day = remaining_days + 1; 
        while month < 12 && day > month_days[month] {
            day -= month_days[month];
            month += 1;
        }
        let month = month + 1;
        let timestamp = format!("{:02}/{:02}/{:04} {:02}:{:02}:{:02}.{:03}", 
                      day, month, year, hours, mins, secs, millis);
        let formatted = format!("[{}][{}] {}\n", timestamp, level.as_str(), message);

        if self.to_stdout {
            print!("{}", formatted);
        }

        if let Some(file) = &self.file {
            let mut file = file.lock().unwrap();
            let _ = file.write_all(formatted.as_bytes());
        }
    }
    pub fn info(&self, msg: &str)  { self.log(LogLevel::Info, msg); }
    pub fn warn(&self, msg: &str)  { self.log(LogLevel::Warn, msg); }
    pub fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
    pub fn debug(&self, msg: &str) { self.log(LogLevel::Debug, msg); }
    pub fn trace(&self, msg: &str) { self.log(LogLevel::Trace, msg); }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).expect("Erro ao ler o arquivo")
    }

    #[test]
    fn test_log_info_written_to_file() {
        let path = "test_log.txt";
        if Path::new(path).exists() {
            fs::remove_file(path).unwrap();
        }
        let logger = Logger::new(LogLevel::Info, Some(path), false);
        logger.info("Mensagem de teste");
        let content = read_file(path);
        assert!(content.contains("INFO"));
        assert!(content.contains("Mensagem de teste"));
        fs::remove_file(path).unwrap(); 
    }

    #[test]
    fn test_log_level_filtering() {
        let path = "test_log_level.txt";
        if Path::new(path).exists() {
            fs::remove_file(path).unwrap();
        }

        let logger = Logger::new(LogLevel::Warn, Some(path), false);
        logger.info("Isto não deve aparecer");
        logger.error("Erro importante");

        let content = read_file(path);
        assert!(!content.contains("Isto não deve aparecer"));
        assert!(content.contains("Erro importante"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_log_formatting() {
        let logger = Logger::new(LogLevel::Debug, None, true);
        logger.debug("Verificando formatação");

        // Aqui só validamos que o método roda sem panic
        // Para testar stdout real, seria necessário redirecionar (mais complexo)
    }
}
