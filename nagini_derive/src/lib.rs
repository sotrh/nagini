use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use std::path::PathBuf;

// struct Include

struct IncludeTypes {
    pipeline: syn::LitStr,
}

impl Parse for IncludeTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse()
    }
}

#[proc_macro]
pub fn include_types(input: TokenStream) -> TokenStream {
    let IncludeTypes { pipeline } = parse_macro_input!(input as IncludeTypes);

    // let path = pipeline.value();
    // let pipeline = nagini_internal::PipelineData::load(path).expect("Unable to load pipeline!");

    todo!()
}

