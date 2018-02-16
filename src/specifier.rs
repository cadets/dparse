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

use ::*;
use std::fmt;

///
/// A DTrace probe name has four optional components, separated by colons:
///
///  - provider
///  - module
///  - function
///  - action
///
/// Each component can be blank; 
///
/// Each is represented here as an `Option<String>`.
///
#[derive(Debug)]
pub struct ProbeSpecifier {
    provider: Option<String>,
    module: Option<String>,
    function: Option<String>,
    action: Option<String>,
}

impl ProbeSpecifier {
    /// Parse a `ProbeSpecifier` from a colon-separated string.
    pub fn parse(s: &str) -> Result<ProbeSpecifier> {
        repack(ProbeSpecifier::nom(s))
    }

    /// Parse a `ProbeSpecifier` using nom.
    named! {
        nom<&str, ProbeSpecifier>,
        do_parse!(
            provider: opt!(ProbeSpecifier::component)
            >> tag_s!(":")
            >> module: opt!(ProbeSpecifier::component)
            >> tag_s!(":")
            >> function: opt!(ProbeSpecifier::component)
            >> tag_s!(":")
            >> action: opt!(complete!(ProbeSpecifier::component))
            >>
            (ProbeSpecifier {
                provider: provider.map(String::from),
                module: module.map(String::from),
                function: function.map(String::from),
                action: action.map(String::from),
            })
        )
    }

    /// Parse a name component (e.g., `arc-*_adjust`).
    named! {
        component<&str,&str>,
        recognize!(take_while1!(|c|
            (c >= 'A' && c <= 'Z')
                || (c >= 'a' && c <= 'z')
                || (c >= '0' && c <= '9')
                || c == '-'
                || c == '_'
                || c == '*'))
    }
}

impl fmt::Display for ProbeSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names = vec!(
            &self.provider,
            &self.module,
            &self.function,
            &self.action,
        );

        write!(f, "{}", names.iter()
            .map(|n| match n {
                &&Some(ref name) => name.to_string(),
                &&None => String::new(),
            })
            .collect::<Vec<_>>()
            .join(":"))
    }
}


#[cfg(test)]
mod tests {
    use ::specifier::*;
    use nom::IResult;
    use std::fmt::Debug;

    fn dump<T: Debug>(res: IResult<&str,T>) {
        match res {
            IResult::Done(rest, value) => {println!("Done {:?} {:?}",rest,value)},
            IResult::Error(err) => {println!("Error {:?}",err)},
            IResult::Incomplete(needed) => {println!("Incomplete {:?}",needed)}
        }
    }

    #[test]
    fn empty_name() {
        dump(ProbeSpecifier::nom(":::"));
        let name = ProbeSpecifier::nom(":::").to_result().expect("parse error");

        assert!(name.provider.is_none());
        assert!(name.module.is_none());
        assert!(name.function.is_none());
        assert!(name.action.is_none());
    }

    #[test]
    fn full_name() {
        let name = ProbeSpecifier::nom("foo:bar:baz:wibble").to_result().expect("parse error");

        assert_eq!(name.provider.expect("provider name"), "foo");
        assert_eq!(name.module.expect("module name"), "bar");
        assert_eq!(name.function.expect("function name"), "baz");
        assert_eq!(name.action.expect("action name"), "wibble");
    }

    #[test]
    fn provider_only() {
        let name = ProbeSpecifier::nom("foo:::").to_result().expect("parse error");

        assert_eq!(name.provider.expect("provider name"), "foo");
        assert!(name.module.is_none());
        assert!(name.function.is_none());
        assert!(name.action.is_none());
    }

    #[test]
    fn wildcards() {
        let name = ProbeSpecifier::nom("perl*:::*-entry").to_result().expect("parse error");

        assert_eq!(name.provider.expect("provider name"), "perl*");
        assert!(name.module.is_none());
        assert!(name.function.is_none());
        assert_eq!(name.action.expect("action name"), "*-entry");
    }
}
