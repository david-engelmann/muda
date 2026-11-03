//! The help and usage texts, pinned byte for byte
//! from the Python's argparse at COLUMNS=80 (the
//! family's standing pin; the Python wraps to the
//! terminal width, the port stays at 80).

pub const TOP_HELP: &str = "\
usage: secondbill [-h] {statement} ...

positional arguments:
  {statement}
    statement  the monthly statement

options:
  -h, --help   show this help message and exit
";

pub const TOP_USAGE: &str = "usage: secondbill [-h] {statement} ...";

pub const STATEMENT_HELP: &str = "\
usage: secondbill statement [-h] --month MONTH [--rates RATES] [-o OUTPUT]
                            repo

positional arguments:
  repo

options:
  -h, --help            show this help message and exit
  --month MONTH
  --rates RATES
  -o OUTPUT, --output OUTPUT
";

pub const STATEMENT_USAGE: &str = "\
usage: secondbill statement [-h] --month MONTH [--rates RATES] [-o OUTPUT]
                            repo";
