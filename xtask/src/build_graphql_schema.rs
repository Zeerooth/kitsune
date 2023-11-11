use async_graphql::{extensions::Tracing, EmptySubscription, Schema};
use std::fs;
use std::path::PathBuf;

use kitsune::http::graphql::{
    GraphQLSchema,
    {mutation::RootMutation, query::RootQuery},
};

pub fn build_graphql_schema(path: PathBuf) -> anyhow::Result<()> {
    info!("Building GraphQL schema...");

    let schema: GraphQLSchema = Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
    .extension(Tracing)
    .finish();

    fs::write(&path, &schema.sdl())?;

    println!("Schema saved to {}", &path.display());

    Ok(())
}
