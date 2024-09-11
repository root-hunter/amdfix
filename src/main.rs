use log::{error, info, warn};

const PARAM_CPU_BOOST_PATH: &str = "/sys/devices/system/cpu/cpufreq/boost";

struct CpuController;
impl CpuController {
    pub fn change_boost_status(value: bool) -> Result<(), std::io::Error> {
        let value: &str = match value {
            true => "1",
            false => "0",
        };

        return std::fs::write(PARAM_CPU_BOOST_PATH, value);
    }

    pub fn get_boost_status() -> Result<bool, std::io::Error> {
        let file = std::fs::read(PARAM_CPU_BOOST_PATH);

        if file.is_ok() {
            let content = file.unwrap();
            let content = content.as_ref();
            let parsed = atoi::atoi::<u8>(content);

            if parsed.is_some() {
                let parsed = parsed.unwrap();

                return match parsed {
                    0 => Ok(false),
                    1 => Ok(true),
                    _ => panic!("Not valid status value (0-1)")
                }
            } else {
                panic!("Not valid content in boost status file")
            }
        } else {
            return Err(file.unwrap_err())
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = clap::Command::new("amdboost")
        .about("CPU AMD settings interface linux")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("boost")
                .about("CPU boost param")
                .subcommand_required(true)
                .arg(
                    clap::Arg::new("flag-path")
                        .long("flag-path")
                        .help("Path of CPU boost flag")
                        .default_value(PARAM_CPU_BOOST_PATH)
                        .action(clap::ArgAction::Set),
                )
                .subcommand(clap::Command::new("active").about("Active CPU boost"))
                .subcommand(clap::Command::new("deactive").about("Deactive CPU boost"))
                .subcommand(clap::Command::new("status").about("Check CPU boost status"))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("boost", boost_matches)) => match boost_matches.subcommand() {
            Some(("active", _)) => {
                let result = CpuController::change_boost_status(true);

                if result.is_ok() {
                    info!("CPU boost activated");
                } else {
                    error!("Error activating CPU boost: {}", result.unwrap_err());
                }
            }
            Some(("deactive", _)) => {
                let result = CpuController::change_boost_status(false);

                if result.is_ok() {
                    info!("CPU boost deactivated");
                } else {
                    error!("Error deactivating CPU boost: {}", result.unwrap_err());
                }
            },
            Some(("status", _)) => {
                let cpu_status = CpuController::get_boost_status();

                if cpu_status.is_ok() {
                    info!("CPU boost status: {}", cpu_status.unwrap());
                } else {
                    error!("Error to retrive CPU boost status");
                }
            }
            _ => {
                warn!("Not valid command");
            }
        },
        _ => {
            warn!("Not valid command");
        }
    }
}
