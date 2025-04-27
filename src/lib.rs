//#!warn(missing_debug_implementation, missing_docs)
mod ast;
mod flags;
mod lexer;
mod parser;
mod tokens;

use std::fmt;

use flags::Flag;

pub type RunFn = fn(args: &[String]) -> Result<(), Error>;

#[derive(Debug)]
enum Error<'a> {
    RunFnNotDefined,
    NoCapForSubs,
    Generic(&'a str),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RunFnNotDefined => write!(f, "Run Function not defined"),
            Error::NoCapForSubs => write!(f, "No Capacity for Sub Commands"),
            Error::Generic(s) => write!(f, "{}", s),
        }
    }
}

/// Used in new function to generate a default [Command]
fn default_run(_: &[String]) -> Result<(), Error> {
    Err(Error::RunFnNotDefined)
}

/// Command struct
#[derive(Debug)]
pub struct Command {
    name: &'static str,
    desc: &'static str,
    long_desc: &'static str,
    run: RunFn,

    subs: Option<&'static mut [&'static mut Command]>,
    subs_idx: usize,
    flags: Option<Vec<&'static mut Flag>>,
    flags_idx: usize,
}

impl Command {
    pub fn new(
        name: &'static str,
        desc: &'static str,
        long_desc: &'static str,
        subs: Option<&'static mut [&'static mut Command]>,
    ) -> Command {
        let mut cmd = Command {
            name,
            desc,
            long_desc,
            flags_idx: 0,
            subs_idx: 0,
            flags: Some(Vec::new()),
            subs: None,
            run: default_run,
        };
        if let Some(_) = subs {
            cmd.subs = subs;
        }
        cmd
    }

    //TODO: to add multiple turn into a macro
    /// Add subs command
    pub fn add_subs(mut self, cmd: &'static mut Command) -> Result<(), Error> {
        if let None = self.subs {
            //TODO: figure out how to handle if not underlaying array is define yet
        }

        tokens::add_command(cmd.name);
        let subs = self.subs.unwrap();

        if self.subs_idx >= subs.len() {
            return Err(Error::NoCapForSubs);
        }

        subs[self.subs_idx] = cmd;
        self.subs_idx += 1;

        Ok(())
    }

    pub fn add_flag(mut self, f: &'static mut Flag) -> Result<(), Error> {
        if let None = self.flags {
            self.flags = Some(Vec::new());
        }

        let mut flags = self.flags.unwrap();
        flags.insert(self.flags_idx, f);

        Ok(())
    }
}

//TODO: start function
