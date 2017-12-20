use libyobicash::errors::YError as LibError;
use libyobicash::errors::YErrorKind as LibErrorKind;
use unqlite::Error as UnQLiteError;
use serde_json::Error as JSONError;
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

    InvalidCoinKind {
        description("Invalid coin kind")
    }

    InvalidCoin {
        description("Invalid coin")
    }

    InvalidRPCMethod {
        description("Invalid message RPCMethod")
    }

    InvalidMessageKind {
        description("Invalid message kind")
    }

    InvalidMessageStatus {
        description("Invalid message status")
    }

    Other(desc: String) {
        description(desc.as_str())
    }
  }
}
