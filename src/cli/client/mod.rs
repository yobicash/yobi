#[derive(StructOpt, Debug)]
#[structopt(name="yobicash-cli", about="The Yobicash client", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
pub enum YobicashClientOpt {
    #[structopt(name="connect", about="Connect to a Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Connect {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
    #[structopt(name="ping", about="Ping a Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Ping {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
    #[structopt(name="info", about="Show info of a Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Info {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
    #[structopt(name="create", about="Create Yobicash resources", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Create {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
        #[structopt(subcommand)]
        cmd: CreateCommands,
    },
    #[structopt(name="push", about="Create and push Yobicash resources", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Push {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
        #[structopt(subcommand)]
        cmd: PushCommands,
    },
    #[structopt(name="send", about="Send Yobicash resources", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Send {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
        #[structopt(subcommand)]
        cmd: SendCommands,
    },
    #[structopt(name="list", about="List Yobicash resources", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    List {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
        #[structopt(subcommand)]
        cmd: ListCommands,
    },
    #[structopt(name="get", about="Get a Yobicash resource", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Get {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
        #[structopt(subcommand)]
        cmd: GetCommands,
    },
    #[structopt(name="mine", about="Mine Yobicash coins", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Mine {
        #[structopt(help="Set the mining difficulty")]
        difficulty: u32,
        #[structopt(short="w", long="wallet", help="Set the wallet where to mine")]
        wallet: String,
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
}

#[derive(StructOpt, Debug)]
pub enum CreateCommands {
    #[structopt(name="wallet", about="Create a Yobicash wallet", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Wallet {
        #[structopt(help="Set the wallet name")]
        name: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum PushCommands {
    #[structopt(name="transaction", about="Create and push a Yobicash transaction", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Transaction {
        #[structopt(help="Set the hex of the transaction to send")]
        hex: Option<String>,
        #[structopt(short="f", long="file", help="Set the path of the file with the hex of the transaction to send")]
        file: Option<String>,
    },
    #[structopt(name="coinbase", about="Create and push a Yobicash coinbase", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coinbase {
        #[structopt(help="Set the hex of the coinbase to send")]
        hex: Option<String>,
        #[structopt(short="f", long="file", help="Set the path of the file with the hex of the coinbase to send")]
        file: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
pub enum SendCommands {
    #[structopt(name="data", about="Create and send a Yobicash data transaction", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Data {
        #[structopt(help="Set the hex of the data to send")]
        hex: Option<String>,
        #[structopt(short="f", long="file", help="Set the path of the file with the hex of the data to send")]
        file: Option<String>,
        #[structopt(short="t", long="to", help="Set the public key hex of the data recipient")]
        to: String,
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the coins")]
        wallet: String,
    },
    #[structopt(name="coins", about="Create and send a Yobicash coins transaction", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coins {
        #[structopt(help="Set the coins amount")]
        amount: u32,
        #[structopt(short="t", long="to", help="Set the public key hex of the coins recipient")]
        to: String,
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the coins")]
        wallet: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum ListCommands {
    #[structopt(name="peers", about="List the Yobicash node peers", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Peers {
        #[structopt(help="Set the wallet from where to get the data", default_value="10")]
        max: u32,
    },
    #[structopt(name="wallets", about="List the Yobicash node wallets", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Wallets,
    #[structopt(name="data", about="List Yobicash data", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Data {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the data")]
        wallet: String,
    },
    #[structopt(name="transactions", about="List a Yobicash wallet transactions", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Transactions {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the transactions")]
        wallet: String,
    },
    #[structopt(name="ancestors", about="List a Yobicash transaction ancestors", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Ancestors {
        #[structopt(long="tx_id", help="Set the id of the descendant transaction")]
        tx_id: String,
    },
    #[structopt(name="coinbases", about="List a Yobicash wallet coinbases", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coinbases {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the coinbases")]
        wallet: String,
    },
    #[structopt(name="coins", about="List a Yobicash wallet coins", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coins {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the coins")]
        wallet: String,
    },
    #[structopt(name="scoins", about="List a Yobicash wallet spent coins", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Scoins {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the spent coins")]
        wallet: String,
    },
    #[structopt(name="ucoins", about="List a Yobicash wallet unspent coins", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Ucoins {
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the unspent coins")]
        wallet: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum GetCommands {
    #[structopt(name="wallet", about="Get a Yobicash wallet", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Wallet {
        #[structopt(help="Set the wallet name")]
        name: String,
    },
    #[structopt(name="data", about="Get a Yobicash data", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Data {
        #[structopt(long="tx_id", help="Set the data tx_id")]
        tx_id: Option<String>,
        #[structopt(long="checksum", help="Set the data checksum")]
        checksum: String,
        #[structopt(long="tag", help="Set the data tag")]
        tag: Option<String>,
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the data")]
        wallet: String,
    },
    #[structopt(name="transaction", about="Get a Yobicash transaction", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Transaction {
        #[structopt(help="Set the transaction id")]
        id: String,
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the transaction")]
        wallet: String,
    },
    #[structopt(name="coinbase", about="Get a Yobicash coinbase", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coinbase {
        #[structopt(help="Set the coinbase id")]
        id: String,
        #[structopt(short="w", long="wallet", help="Set the wallet from where to get the coinbase")]
        wallet: String,
    },
    #[structopt(name="coin", about="Get a Yobicash coin", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Coin {
        #[structopt(long="tx_id", help="Set the coin tx_id")]
        tx_id: String,
        #[structopt(long="idx", help="Set the coin idx")]
        idx: u32,
    }
}
