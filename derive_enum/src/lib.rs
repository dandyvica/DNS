use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident, Variant};

use proc_macro::TokenStream;

//--------------------------------------------------------------------------------
// Implement the FromStr trait for C-style enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(FromStr)]
pub fn from_str(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => impl_from_str(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    //println!("{}", code);
    code.into()
}

fn impl_from_str(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
    let enum_name = &ast.ident;

    let arms = de.variants.iter().map(|v| build_fromstr_arm(enum_name, v));

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics std::str::FromStr for #enum_name #ty_generics #where_clause {
            type Err = &'static str;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    #( #arms)*
                    _ => Err("no corresponding variant"),
                }
            }
        }
    }
}

fn build_fromstr_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let variant_ident_as_string = &variant.ident.to_string();

    // this only works for C-style enums
    if matches!(&variant.fields, Fields::Unit) {
        quote! {
            #variant_ident_as_string => Ok(#enum_name::#variant_ident),
        }
    } else {
        unimplemented!("only C-style enums are implemented")
    }
}

//--------------------------------------------------------------------------------
// Implement the TryFrom trait for C-style enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(TryFrom)]
pub fn try_from(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => impl_try_from(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    //println!("{}", code);
    code.into()
}

// enum is either u8 or u16
#[derive(Debug)]
enum ReprSize {
    U8,
    U16,
}

fn impl_try_from(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
    let enum_name = &ast.ident;
    let repr_size = repr_size(ast);

    let arms = de
        .variants
        .iter()
        .map(|v| build_tryfrom_arm(enum_name, v, &repr_size));

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    match repr_size {
        ReprSize::U8 => {
            quote! {
                impl #impl_generics std::convert::TryFrom<u8> for #enum_name #ty_generics #where_clause {
                    type Error = &'static str;

                    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
                        match value {
                            #( #arms)*
                            _ => Err("no variant corresponding to value"),
                        }
                    }
                }
            }
        }
        ReprSize::U16 => {
            quote! {
                impl #impl_generics std::convert::TryFrom<u16> for #enum_name #ty_generics #where_clause {
                    type Error = &'static str;

                    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
                        match value {
                            #( #arms)*
                            _ => Err("no variant corresponding to value"),
                        }
                    }
                }
            }
        }
    }
}

fn build_tryfrom_arm(
    enum_name: &Ident,
    variant: &Variant,
    size: &ReprSize,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    match size {
        ReprSize::U8 => {
            quote! {
                x if x == #enum_name::#variant_ident as u8 => Ok(#enum_name::#variant_ident),
            }
        }
        ReprSize::U16 => {
            quote! {
                x if x == #enum_name::#variant_ident as u16 => Ok(#enum_name::#variant_ident),
            }
        }
    }
}

// get the repr size (u8 or u16 because I only use those)
// parse_nested_meta() -> Result<(), syn::parse::Error>
// I'd like to return: Result<ReprSize, syn::parse::Error>
fn repr_size(ast: &DeriveInput) -> ReprSize {
    // look for all attributes and search for repr with a supported size
    let mut repr_size = ReprSize::U8;

    for attr in ast.attrs.iter() {
        if attr.path().is_ident("repr") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("u8") {
                    return Ok(());
                }

                if meta.path.is_ident("u16") {
                    repr_size = ReprSize::U16;
                    return Ok(());
                }

                Err(meta.error("unsupported repr size"))
            })
            .unwrap();
        }
    }

    repr_size
}
