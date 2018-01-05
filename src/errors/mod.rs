use libyobicash::errors::YError as LibError;
use libyobicash::errors::YErrorKind as LibErrorKind;
use unqlite::Error as UnQLiteError;
use serde_json::Error as JSONError;
use std::string::FromUtf8Error;
use std::io::Error as IOError;

error_chain! {
  types {
    YHError, YHErrorKind, YHResultExt, YHResult;
  }

  links {
    Lib(LibError, LibErrorKind);
  }

  foreign_links {
    IO(IOError);
    Store(UnQLiteError);
    JSON(JSONError);
    String(FromUtf8Error);
  }

  errors {
    InvalidPassword {
        description("Invalid password")
    }

    InvalidKey {
        description("Invalid key")
    }

    InvalidLength {
        description("Invalid length")
    }

    InvalidValue {
        description("Invalid value") 
    }

    UnknownValue {
        description("Unknown value")
    }

    NotEnoughFunds {
        description("Not enough funds")
    }
    
    NotFound {
        description("Not found")
    }

    AlreadyFound {
        description("Already found")
    }

    InvalidLevel {
        description("Invalid level")
    }

    ParsingFailure {
        description("Parsing failure")
    }

    InvalidDifficulty {
        description("Invalid difficulty")
    }

    InvalidCoinKind {
        description("Invalid coin kind")
    }

    InvalidCoin {
        description("Invalid coin")
    }

    InvalidRPCMethod {
        description("Invalid message rpc method")
    }

    InvalidMessagePrefix {
        description("Invalid message prefix")
    }

    InvalidMessageKind {
        description("Invalid message kind")
    }

    InvalidMessageStatus {
        description("Invalid message status")
    }

    InvalidRequest {
        description("Invalid request")
    }

    InvalidResponse {
        description("Invalid response")
    }

    InvalidIp {
        description("Invalid ip")
    }

    MaxConnectionsReached {
        description("Max connections reached")
    }

    FailedConnection {
        description("Failed connection")
    }

    NotConnected {
        description("Not connected")
    }

    Other(desc: String) {
        description(desc.as_str())
    }
  }
}
