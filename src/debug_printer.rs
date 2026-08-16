pub struct DebugPrinter;

impl DebugPrinter {
    pub fn log_state(message: String) {
        log::debug!("{}", message);
    }

    pub fn log_action(module: String, action: String) {
        log::trace!("{}: {}", module, action);
    }

    pub fn log_info(information: String) {
        log::info!("{}", information);
    }
}
