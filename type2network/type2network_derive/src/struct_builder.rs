use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, Index};

pub struct StructDeriveBuilder;
pub type StructBuilder = fn(&DeriveInput, &DataStruct) -> proc_macro2::TokenStream;

impl StructDeriveBuilder {
    pub fn to_network(ast: &DeriveInput, ds: &DataStruct) -> proc_macro2::TokenStream {
        // no code is created for unit structs
        if StructDeriveBuilder::is_unit(ds) {
            return quote!();
        }

        let struct_name = &ast.ident;

        let method_calls = ds.fields.iter().enumerate().map(|field| {
            match &field.1.ident {
                // case of a struct with named fields
                Some(field_name) => {
                    quote! {
                        length += ToNetworkOrder::to_network_order(&self.#field_name, buffer)?;
                    }
                }
                // case of a tuple struct
                None => {
                    let index = Index::from(field.0);
                    quote! {
                        length += ToNetworkOrder::to_network_order(&self.#index, buffer)?;
                    }
                }
            }
        });

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics ToNetworkOrder for #struct_name #ty_generics #where_clause {
                fn to_network_order<W: std::io::Write>(&self, buffer: &mut W) -> Result<usize> {
                    let mut length = 0usize;
                    #( #method_calls)*
                    Ok(length)
                }
            }
        }
    }

    pub fn from_network(ast: &DeriveInput, ds: &DataStruct) -> proc_macro2::TokenStream {
        // no code is created for unit structs
        if StructDeriveBuilder::is_unit(ds) {
            return quote!();
        }

        let struct_name = &ast.ident;

        // call from_network_order() call for each field
        let method_calls = ds.fields.iter().enumerate().map(|field| {
            match &field.1.ident {
                // case of a struct with named fields
                Some(field_name) => {
                    quote! {
                        FromNetworkOrder::from_network_order(&mut self.#field_name, buffer)?;
                    }
                }
                // case of a tuple struct
                None => {
                    let index = Index::from(field.0);
                    quote! {
                        FromNetworkOrder::from_network_order(&mut self.#index, buffer)?;
                    }
                }
            }
        });

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics FromNetworkOrder for #struct_name #ty_generics #where_clause {
                fn from_network_order<R: std::io::Read>(&mut self, buffer: &mut R) -> Result<()> {
                    #( #method_calls)*
                    Ok(())
                }
            }
        }
    }

    // Test whether the struct is a unit struct
    fn is_unit(ds: &DataStruct) -> bool {
        matches!(ds.fields, Fields::Unit)
    }
}
