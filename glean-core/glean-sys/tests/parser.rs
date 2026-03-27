use std::fs;

use xshell::{Shell, cmd};

#[test]
fn generated_metrics_code_up_to_date() {
    // Relative to `glean-sys`
    let udl_file = "../src/glean.udl";
    let udl_src = fs::read_to_string(udl_file).expect("unable to read UDL file");

    let output = glean_sys_parser::generate(&udl_src);
    let dst_file = "src/metrics.rs";

    fs::write(dst_file, output.as_bytes()).unwrap();

    // Last but not least check if we modified the document.
    let sh = Shell::new().unwrap();
    cmd!(sh, "git --no-pager diff --exit-code {dst_file}")
        .run()
        .unwrap();
}
