use structopt::StructOpt;
use cli::server::opt::*;
use errors::*;

pub struct YServerCli;

impl YServerCli {
    pub fn from_args() -> YHResult<()> {
        let opt = YServerOpt::from_args();
        match opt {
            YServerOpt::Start { light, temporary, difficulty, verbose } => {
                YServerCli::start(light, temporary, difficulty, verbose)
            }
        }
    }

    pub fn start(light: bool, temporary: bool, difficulty: Option<u32>, verbose: bool) -> YHResult<()> {
        unreachable!()
    }
}
