use proc_macro::TokenStream;

mod include_puzzles;

#[proc_macro]
pub fn include_puzzles(input: TokenStream) -> TokenStream {
    include_puzzles::include_puzzles(input.into())
        .unwrap_or_else(|error| panic!("{}", error.to_string()))
        .into()
}
