use libyobicash::errors::YError as LibError;
use libyobicash::errors::YErrorKind as LibErrorKind;
use unqlite::Error as UnQLiteError;
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
  }

  errors {
    NotFound {
        description("Not found")
    }

    AlreadyFound {
        description("Already found")
    }

    InvalidCoinKind {
        description("Invalid coin kind")
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
