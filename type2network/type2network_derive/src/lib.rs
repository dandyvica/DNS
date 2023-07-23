//use syn::visit::{self, Visit};
use syn::{parse_macro_input, Data, DeriveInput};

use proc_macro::TokenStream;
//use quote::{quote, ToTokens};

mod struct_builder;
use struct_builder::{StructBuilder, StructDeriveBuilder};

mod enum_builder;
use enum_builder::{EnumBuilder, EnumDeriveBuilder};

#[proc_macro_derive(ToNetwork)]
pub fn to_network(input: TokenStream) -> TokenStream {
    derive_helper(
        input,
        StructDeriveBuilder::to_network,
        EnumDeriveBuilder::to_network,
    )
}

#[proc_macro_derive(FromNetwork)]
pub fn from_network(input: TokenStream) -> TokenStream {
    derive_helper(
        input,
        StructDeriveBuilder::from_network,
        EnumDeriveBuilder::from_network,
    )
}

fn derive_helper(
    input: TokenStream,
    struct_builder: StructBuilder,
    enum_builder: EnumBuilder,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => enum_builder(&ast, de),
        Data::Struct(ds) => struct_builder(&ast, ds),
        _ => unimplemented!("{} is neither a struct, nor an enum", ast.ident.to_string()),
    };

    println!("code ============> '{}'", code);

    code.into()
}
