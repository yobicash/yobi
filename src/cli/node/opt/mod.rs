#[derive(StructOpt, Debug)]
#[structopt(name="yobicashd", about="The Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
pub enum YNodeOpt {
    #[structopt(name="start", about="Start the Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Start {
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        #[structopt(long="light", help="Use light store")]
        light: bool,
        #[structopt(long="temporary", help="Use temporary store")]
        temporary: bool,
        #[structopt(short="m", long="mine", help="Activate mining")]
        difficulty: Option<u32>,
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
