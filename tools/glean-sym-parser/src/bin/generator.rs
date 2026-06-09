// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let udl_file = args.next().expect("need path to UDL file");
    let udl_src = fs::read_to_string(udl_file).expect("unable to read UDL file");

    let output = glean_sym_parser::generate(&udl_src);
    print!("{output}");
}
