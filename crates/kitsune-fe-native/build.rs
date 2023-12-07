fn main() {
    cynic_codegen::register_schema("kitsune")
        .from_sdl_file("../../schemas/kitsune.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
