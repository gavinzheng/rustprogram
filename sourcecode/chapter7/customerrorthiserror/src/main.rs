use std::{error::Error, fmt::Debug};

fn main() {
    println!("{:?}", make_request("key.txt").unwrap_err());
}

#[derive(thiserror::Error)]
enum CustomError {
    #[error("the length of filename should be 10, but was {}", .0.len())]
    ValidationError(String),

    #[error("failed to read the key file")]
    FileReadError(#[source] std::io::Error),

    #[error("failed to send the api request")]
    RequestError(#[from] reqwest::Error),

    #[error("failed to delete the key file")]
    FileDeleteError(#[source] std::io::Error),
}

fn make_request(filename: &str) -> Result<(), CustomError> {
    use CustomError::*;

    let filename = if filename.len() == 0 {
        filename
    } else {
        return Err(ValidationError(filename.into()));
    };
    let key = std::fs::read_to_string(filename).map_err(FileReadError)?;
    reqwest::blocking::get(format!("http:key/{}", key))?.error_for_status()?;
    std::fs::remove_file(filename).map_err(FileDeleteError)?;
    Ok(())
}

// impl From<reqwest::Error> for CustomError {
//     fn from(e: reqwest::Error) -> Self {
//         CustomError::RequestError(e)
//     }
// }

// impl std::error::Error for CustomError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             CustomError::FileReadError(s) => Some(s),
//             CustomError::RequestError(s) => Some(s),
//             CustomError::FileDeleteError(s) => Some(s),
//         }
//     }
// }

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

// impl std::fmt::Display for CustomError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CustomError::FileReadError(_) => write!(f, "failed to read the key file"),
//             CustomError::RequestError(_) => write!(f, "failed to send the api request"),
//             CustomError::FileDeleteError(_) => write!(f, "failed to delete the key file"),
//         }
//     }
// }