// Version handler - prints build information from shared constants

use crate::presentation::cli::output;
use crate::shared::kernel::result::AppResult;

pub(crate) fn run() -> AppResult<()> {
    let info = output::VersionInfo::current();
    output::print_version(&info);
    Ok(())
}
