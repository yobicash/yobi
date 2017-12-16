#[derive(StructOpt, Debug)]
#[structopt(name="yobicashd", about="The Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
pub enum YobicashdOpt {
    #[structopt(name="start", about="Start the Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Start {
        #[structopt(short="H", long="host", help="Set a custom host")]
        host: Option<String>,
        #[structopt(short="p", long="port", help="Set a custom port")]
        port: Option<u16>,
        #[structopt(short="S", long="seed", help="Set a custom seed file path")]
        seed: Option<String>,
        #[structopt(short="C", long="config", help="Set a custom config file path")]
        config: Option<String>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
    #[structopt(name="status", about="Show the status of the Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Status {
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
    #[structopt(name="stop", about="Stop the Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Stop {
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    }
}
