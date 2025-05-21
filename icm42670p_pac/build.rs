use regcommsgen::{
    generate_src_dir,
    read_peripheral_spec,
};


const PSPEC_PATH: &'static str = "icm42670p.yaml";
fn main() {
    println!("cargo:rerun_if_changed={}", PSPEC_PATH);

    println!("Regenerating quantum_flux_sensor source");
    let pspec = read_peripheral_spec(&PSPEC_PATH);
    generate_src_dir(&pspec, "./src");
}
