extern crate failure;
extern crate os_type;
extern crate serde;
extern crate tempdir;
extern crate toml;
extern crate uuid;

use self::failure::Error;
use self::uuid::Uuid;
use backtrace::Backtrace;
use std::borrow::Cow;
use std::{env, fs::File, io::Write, path::Path, path::PathBuf};

/// Method of failure.
#[derive(Debug, Serialize, Clone, Copy)]
pub enum Method {
  Panic,
}

#[derive(Debug, Serialize)]
pub struct Report {
  name: String,
  operating_system: Cow<'static, str>,
  crate_version: String,
  explanation: String,
  cause: String,
  method: Method,
  backtrace: String,
}

impl Report {
  /// Create a new instance.
  pub fn new(
    name: &str,
    version: &str,
    method: Method,
    explanation: String,
    cause: String,
  ) -> Self {
    let operating_system = if cfg!(windows) {
      "windows".into()
    } else {
      let platform = os_type::current_platform();
      format!("unix:{:?}", platform.os_type).into()
    };

    let backtrace = format!("{:#?}", Backtrace::new());

    Self {
      crate_version: version.into(),
      name: name.into(),
      operating_system,
      method,
      explanation,
      cause,
      backtrace,
    }
  }

  pub fn serialize(&self) -> Option<String> {
    toml::to_string_pretty(&self).ok()
  }

  /// Write a file to disk.
  pub fn persist(&self) -> Result<PathBuf, Error> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let tmp_dir = env::temp_dir();
    let tmp_dir = match tmp_dir.to_str() {
      Some(dir) => dir,
      None => bail!("Could not create a tmp directory for a report."),
    };
    let file_name = format!("report-{}.toml", &uuid);
    let file_path = Path::new(tmp_dir).join(file_name);
    let mut file = File::create(&file_path)?;
    let toml = self.serialize().unwrap();
    file.write_all(toml.as_bytes())?;
    Ok(file_path)
  }
}
