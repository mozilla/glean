// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;

use xshell::{Shell, cmd};

#[test]
fn generated_metrics_code_up_to_date() {
    // Relative to `glean-sym`
    let udl_file = "../src/glean.udl";
    let udl_src = fs::read_to_string(udl_file).expect("unable to read UDL file");

    let output = glean_sym_parser::generate(&udl_src);
    let dst_file = "src/metrics.rs";

    fs::write(dst_file, output.as_bytes()).unwrap();

    // Last but not least check if we modified the document.
    let sh = Shell::new().unwrap();
    cmd!(sh, "git --no-pager diff --exit-code {dst_file}")
        .run()
        .unwrap();
}
