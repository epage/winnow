mod json;
mod parser;
mod parser_dispatch;
#[allow(dead_code)]
mod parser_streaming;

use winnow::error::convert_error;
use winnow::error::Error;
use winnow::error::VerboseError;
use winnow::prelude::*;

fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    let data = args.input.as_deref().unwrap_or(if args.invalid {
        "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { 1\"hello\" : \"world\"
  }
  } "
    } else {
        "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  } "
    });

    if args.verbose {
        match parser::json::<VerboseError<&str>>(data).finish() {
            Ok(json) => {
                println!("{:#?}", json);
            }
            Err(err) => {
                if args.pretty {
                    println!("{}", convert_error(data, err));
                } else {
                    println!("{:#?}", err);
                }
            }
        }
    } else {
        let result = match args.implementation {
            Impl::Naive => parser::json::<Error<&str>>(data).finish(),
            Impl::Dispatch => parser_dispatch::json::<Error<&str>>(data).finish(),
        };
        match result {
            Ok(json) => {
                println!("{:#?}", json);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    Ok(())
}

#[derive(Default)]
struct Args {
    input: Option<String>,
    invalid: bool,
    verbose: bool,
    pretty: bool,
    implementation: Impl,
}

enum Impl {
    Naive,
    Dispatch,
}

impl Default for Impl {
    fn default() -> Self {
        Self::Naive
    }
}

impl Args {
    fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut res = Args::default();

        let mut args = lexopt::Parser::from_env();
        while let Some(arg) = args.next()? {
            match arg {
                Long("invalid") => {
                    res.invalid = true;
                }
                Long("verbose") => {
                    res.verbose = true;
                    // Only case where verbose matters
                    res.invalid = true;
                }
                Long("pretty") => {
                    res.verbose = true;
                    // Only case where pretty matters
                    res.pretty = true;
                    res.invalid = true;
                }
                Long("impl") => {
                    res.implementation = args.value()?.parse_with(|s| match s {
                        "naive" => Ok(Impl::Naive),
                        "dispatch" => Ok(Impl::Dispatch),
                        _ => Err("expected `naive`, `dispatch`"),
                    })?;
                }
                Value(input) => {
                    res.input = Some(input.string()?);
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(res)
    }
}
