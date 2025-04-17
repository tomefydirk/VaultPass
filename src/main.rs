mod vaultpass;
use vaultpass::run_vaultpass;
fn main() -> Result<(), eframe::Error> {
    run_vaultpass()
}
