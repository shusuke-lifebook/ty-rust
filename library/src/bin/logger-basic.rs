use log::{debug, error, info, trace, warn};

fn main() {
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    trace!("traceログ");
    debug!("debugログ");
    info!("infoログ");
    warn!("warnログ");
    error!("errorログ");
}
