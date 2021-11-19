use std::fmt;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub struct EmacsClient {
    socket_path: String,
}

#[derive(Debug)]
pub enum EmacsError {
    IoError(std::io::Error),
    EvalError(String),
}

impl std::error::Error for EmacsError {}

impl fmt::Display for EmacsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmacsError::IoError(err) => err.fmt(f),
            EmacsError::EvalError(msg) => write!(f, "eval error: {}", msg),
        }
    }
}

impl From<std::io::Error> for EmacsError {
    fn from(err: std::io::Error) -> Self {
        EmacsError::IoError(err)
    }
}

impl EmacsClient {
    pub fn new(socket_path: &str) -> EmacsClient {
        EmacsClient {
            socket_path: socket_path.to_owned(),
        }
    }

    pub fn eval(&mut self, expr: &str) -> Result<String, EmacsError> {
        let mut sock = UnixStream::connect(&self.socket_path)?;

        let cmd = format!("-current-frame -eval {} \n", Self::quote_argument(expr));
        sock.write_all(cmd.as_bytes())?;
        sock.flush()?;

        let mut response = String::new();
        // TODO: reading an error somehow blocks for a few seconds.
        sock.read_to_string(&mut response)?;

        for line in response.lines() {
            if let Some(value) = line.strip_prefix("-print ") {
                return Ok(Self::unquote_argument(value));
            } else if let Some(value) = line.strip_prefix("-error ") {
                return Err(EmacsError::EvalError(Self::unquote_argument(value)));
            }
        }

        panic!(
            "expected response to contain '-print' or '-error' on eval of: {}, got: {}",
            expr, response
        );
    }

    /// Quote an argument to the Emacs server.
    ///
    /// Inserts a '&' before each '&', each space, each newline and any initial '-'.
    /// Changes space to underscores, too, so that the return value never contains a
    /// space.
    ///
    /// https://github.com/emacs-mirror/emacs/blob/cde5dcd441b5db79f39b8664221866566c400b05/lib-src/emacsclient.c#L828
    fn quote_argument(s: &str) -> String {
        let mut chars = s.chars();
        let start = match chars.next() {
            Some('-') => "&-".to_owned(),
            Some(ch) => ch.to_string(),
            None => "".to_owned(),
        };

        start
            + &chars
                .flat_map(|c| match c {
                    ' ' => vec!['&', '_'],
                    '\n' => vec!['&', 'n'],
                    '&' => vec!['&', '&'],
                    _ => vec![c],
                })
                .collect::<String>()
    }

    /// Unquote an argument from the Emacs server.
    fn unquote_argument(s: &str) -> String {
        let mut chars = s.chars();
        let mut out = String::new();
        while let Some(ch) = chars.next() {
            if ch == '&' {
                match chars.next() {
                    Some('_') => out.push(' '),
                    Some('n') => out.push('\n'),
                    Some(ch) => out.push(ch),
                    None => panic!("unexpected EOF after escape char '&' in: {}", s),
                }
            } else {
                out.push(ch);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_argument() {
        assert_eq!(
            EmacsClient::quote_argument("(message \"test\")"),
            "(message&_\"test\")"
        );
    }

    #[test]
    fn test_unquote_argument() {
        assert_eq!(EmacsClient::unquote_argument("(a&_+&_1)&n"), "(a + 1)\n");
    }
}
