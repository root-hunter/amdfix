use log::{error, info, warn};

const PARAM_CPU_BOOST_PATH: &str = "/sys/devices/system/cpu/cpufreq/boost";

struct CpuController;
impl CpuController {
    pub fn change_boost(value: bool) -> Result<(), std::io::Error> {
        let v: &str = match value {
            true => "1",
            false => "0"
        };

        let result = std::fs::write(PARAM_CPU_BOOST_PATH, v);

        if result.is_ok() {
            return Ok(());
        } else {
            let err = result.unwrap_err();
            return Err(err);
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
        .subcommand(clap::Command::new("boost")
            .about("CPU boost param")
            .subcommand_required(true)
            .arg(
                clap::Arg::new("flag-path")
                    .long("flag-path")
                    .help("Path of CPU boost flag")
                    .default_value(PARAM_CPU_BOOST_PATH)
                    .action(clap::ArgAction::Set)   
            )
            .subcommand(
                clap::Command::new("active")
                    .about("Active CPU boost")
            )
            .subcommand(
                clap::Command::new("deactive")
                    .about("Deactive CPU boost")
            )).get_matches();

    match matches.subcommand() {
        Some(("boost", boost_matches)) => {
            let active_boost = boost_matches.get_flag("active");
            let result = CpuController::change_boost(active_boost);

            if result.is_ok() {
                if active_boost {
                    info!("CPU boost activated");
                } else {
                    info!("CPU boost deactivated");
                }
            } else {
                error!("Error updating CPU boost: {}", result.unwrap_err());
            }
        },
        _ => {
            warn!("Not valid command");
        }
    }
}
