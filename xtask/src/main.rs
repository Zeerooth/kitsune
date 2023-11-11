#[macro_use]
extern crate tracing;

use std::path::PathBuf;

mod build_graphql_schema;
mod build_scss;

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "build-scss")]
/// Build a directory of SCSS files
struct BuildScss {
    #[argh(option, default = "\"kitsune/assets\".into()")]
    /// path to the directory
    path: PathBuf,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "build-graphql-schema")]
/// Build a graphql schema
struct BuildGraphqlSchema {
    #[argh(option, default = "\"schema/schema.graphql\".into()")]
    /// path to the file
    path: PathBuf,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    BuildScss(BuildScss),
    BuildGraphqlSchema(BuildGraphqlSchema),
}

#[derive(argh::FromArgs)]
/// Kitsune dev taskrunner
struct Command {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let command: Command = argh::from_env();
    match command.subcommand {
        Subcommand::BuildScss(BuildScss { path }) => build_scss::build_scss(path)?,
        Subcommand::BuildGraphqlSchema(BuildGraphqlSchema { path }) => {
            build_graphql_schema::build_graphql_schema(path)?
        }
    }

    Ok(())
}
