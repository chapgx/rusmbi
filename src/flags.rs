#[derive(Debug)]
pub struct Flag {
    name: &'static str,
    short: &'static str,
    desc: &'static str,
    long_desc: &'static str,
}

impl Flag {
    pub fn new(
        name: &'static str,
        short: &'static str,
        desc: &'static str,
        long_desc: &'static str,
    ) -> Flag {
        let f = Flag {
            name,
            short,
            desc,
            long_desc,
        };
        f
    }
}
