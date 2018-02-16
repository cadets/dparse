/*
 * Copyright (c) 2018 Jonathan Anderson
 * All rights reserved.
 *
 * This software was developed by BAE Systems, the University of Cambridge
 * Computer Laboratory, and Memorial University under DARPA/AFRL contract
 * FA8650-15-C-7558 ("CADETS"), as part of the DARPA Transparent Computing
 * (TC) research program.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#[macro_use]
extern crate clap;
extern crate dparse;

use std::io::Read;


fn main() {
    let cli_yaml = load_yaml!("dparse.yaml");
    let args = clap::App::from_yaml(cli_yaml).get_matches();

    match args.values_of("SCRIPT") {
        None => {
            println!("No script(s) specified");
            assert!(false, "SCRIPT ought to have been marked `required` in bin/dparse.yaml");
        },
        Some(filenames) => {
            for filename in filenames {
                match parse_file(filename) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error parsing '{}': {}", filename, e);
                        break;
                    },
                }
            }
        },
    }
}


fn parse_file(filename: &str) -> Result<(), dparse::Error> {
    let mut file = std::fs::File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let spec = dparse::ProbeSpecifier::parse(&content)?;
    println!("{}\n", spec);

    Ok(())
}
