#[derive(StructOpt, Debug)]
#[structopt(name="yobicash-server", about="The Yobicash low-level node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
pub enum YServerOpt {
    #[structopt(name="start", about="Start the Yobicash node server", version="0.1.0", author="Christian Nyumbayire <christian@yobicash.org>")]
    Start {
        #[structopt(long="light", help="Use light store")]
        light: bool,
        #[structopt(long="temporary", help="Use temporary store")]
        temporary: bool,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        #[structopt(short="m", long="mine", help="Activate mining")]
        difficulty: Option<u32>,
        #[structopt(short="v", long="verbose", help="Activate verbose mode")]
        verbose: bool,
    },
}

