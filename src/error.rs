// The MIT License (MIT)
//
// Copyright (c) 2016 Siphilia
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::error;
use std::fmt;
use std::io;
use std::process;
use std::result;
use yaml_rust as yaml;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    LoadYaml(yaml::ScanError),
    EmitYaml(yaml::EmitError),
    ConfigDirIsNotFound,
    EnvSetExistsWhenInitializing,
    ProcessFail(process::ExitStatus),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref e) => write!(f, "IO Error ({})", e),
            Error::LoadYaml(ref e) => write!(f, "Yaml Scan Error ({})", e),
            Error::EmitYaml(ref e) => write!(f, "Yaml Emit Error ({:?})", e),
            Error::ConfigDirIsNotFound => write!(f, "Config dir is not found"),
            Error::EnvSetExistsWhenInitializing => {
                write!(f, "EnvSet file exists when initializing")
            }
            Error::ProcessFail(ref status) => {
                write!(f, "Process failed ({})", status)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(a: io::Error) -> Error {
        Error::IO(a)
    }
}

impl From<yaml::ScanError> for Error {
    fn from(a: yaml::ScanError) -> Error {
        Error::LoadYaml(a)
    }
}

impl From<yaml::EmitError> for Error {
    fn from(a: yaml::EmitError) -> Error {
        Error::EmitYaml(a)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref e) => e.description(),
            Error::LoadYaml(ref e) => e.description(),
            Error::EmitYaml(_) => "Error in YamlEmitter.",
            Error::ConfigDirIsNotFound => "Config dir is not found.",
            Error::EnvSetExistsWhenInitializing => "A EnvSet file exists when Initializing.",
            Error::ProcessFail(_) => "Running child process was fail.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref e) => Some(e),
            Error::LoadYaml(ref e) => Some(e),
            Error::EmitYaml(_) |
            Error::ConfigDirIsNotFound |
            Error::EnvSetExistsWhenInitializing |
            Error::ProcessFail(_) => None,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
