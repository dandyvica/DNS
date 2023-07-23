//use proc_macro2::Span;
use quote::quote;
use syn::{DataEnum, DeriveInput};

pub struct EnumDeriveBuilder;
pub type EnumBuilder = fn(&DeriveInput, &DataEnum) -> proc_macro2::TokenStream;

// enum is either u8 or u16
#[derive(Debug)]
enum ReprSize {
    U8,
    U16,
}

impl EnumDeriveBuilder {
    pub fn to_network(ast: &DeriveInput, _de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;

        // let arms = de
        //     .variants
        //     .iter()
        //     .map(|v| EnumDeriveBuilder::build_variant_to_arm(enum_name, v));

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        // let code = quote! {
        //     impl #impl_generics ToNetworkOrder for #enum_name #ty_generics #where_clause {
        //         fn to_network_order<W: std::io::Write>(&self, buffer: &mut W) -> Result<usize> {
        //             match self {
        //                 #( #arms)*
        //             }
        //         }
        //     }
        // };

        match EnumDeriveBuilder::repr_size(ast) {
            ReprSize::U8 => {
                quote! {
                    impl #impl_generics ToNetworkOrder for #enum_name #ty_generics #where_clause {
                        fn to_network_order<W: std::io::Write>(&self, buffer: &mut W) -> Result<usize> {
                            buffer.write_u8::<BigEndian>(*self as u8)?;
                            Ok(1)
                        }
                    }
                }
            }
            ReprSize::U16 => {
                quote! {
                    impl #impl_generics ToNetworkOrder for #enum_name #ty_generics #where_clause {
                        fn to_network_order<W: std::io::Write>(&self, buffer: &mut W) -> Result<usize> {
                            buffer.write_u16::<BigEndian>(*self as u16)?;
                            Ok(2)
                        }
                    }
                }
            }
        }
    }

    // Test whether all enum variant are unit
    // fn is_unit_only(_ast: &DeriveInput, de: &DataEnum) -> bool {
    //     de.variants.iter().all(|v| matches!(v.fields, Fields::Unit))
    // }

    // Build the code for each variant arm
    // Ex: if enum is:
    //
    // #[repr(u8)]
    // enum Message {
    //     Ok = 0,
    //     Quit = 1,
    //     Move { x: u16, y: u16 },
    //     Write(String),
    //     ChangeColor(u16, u16, u16),
    // }
    //
    // then this function will build the arm for the variant passed as the 2nd parameter.
    // Ex:
    //
    // Message::ChangeColor(f0, f1, f2) => {
    //        let mut length = 0usize ;
    //        length += ToNetworkOrder ::to_network_order(f0, buffer)?;
    //        length += ToNetworkOrder ::to_network_order(f1, buffer)?;
    //        length += ToNetworkOrder ::to_network_order(f2, buffer)?;
    //        Ok(length)
    // },
    // fn build_variant_to_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    //     let variant_ident = &variant.ident;

    //     match &variant.fields {
    //         // unnamed variant like: ChangeColor(i32, i32, i32)
    //         Fields::Unnamed(_) => {
    //             let field_names = (0..variant.fields.len())
    //                 .map(|i| Ident::new(&format!("f{}", i), Span::call_site()));

    //             let method_calls = field_names.clone().map(|f| {
    //                 quote! {
    //                     length += ToNetworkOrder::to_network_order(#f, buffer)?;
    //                 }
    //             });

    //             quote! {
    //                 #enum_name::#variant_ident(#(#field_names),*) => {
    //                     let mut length = 0usize;
    //                     #( #method_calls)*
    //                     Ok(length)
    //                 },
    //             }
    //         }
    //         // named variant like: Move { x: i32, y: i32 }
    //         Fields::Named(_) => {
    //             let members = variant.fields.iter().map(|f| &f.ident);

    //             let method_calls = members.clone().map(|f| {
    //                 quote! {
    //                     length += ToNetworkOrder::to_network_order(#f, buffer)?;
    //                 }
    //             });

    //             quote! {
    //                 #enum_name::#variant_ident{ #(#members),* } => {
    //                     let mut length = 0usize;
    //                     #( #method_calls)*
    //                     Ok(length)
    //                 },
    //             }
    //         }
    //         // unit variant like: Quit = 1
    //         Fields::Unit => {
    //             quote! {
    //                 #enum_name::#variant_ident => {
    //                     let value = #enum_name::#variant_ident;
    //                     let size = std::mem::size_of_val(&value);
    //                     match size {
    //                         1 => buffer.write_u8(value as u8)?,
    //                         2 => buffer.write_u16::<BigEndian>(value as u16)?,
    //                         4 => buffer.write_u32::<BigEndian>(value as u32)?,
    //                         8 => buffer.write_u64::<BigEndian>(value as u64)?,
    //                         _ => unimplemented!("size of variant is not supported"),
    //                     }

    //                     Ok(size)
    //                 },
    //             }
    //         }
    //     }
    // }

    pub fn from_network(ast: &DeriveInput, _de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;
        let repr_size = EnumDeriveBuilder::repr_size(ast);

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        match repr_size {
            ReprSize::U8 => quote! {
                impl #impl_generics FromNetworkOrder for #enum_name #ty_generics #where_clause {
                    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
                        let value = buffer.read_u8()?;
                        match <#enum_name>::try_from(value) {
                            Ok(ct) => {
                                *self = ct;
                                Ok(())
                            }
                            _ => Err(Error::from(ErrorKind::NotFound)),
                        }
                    }
                }
            },
            ReprSize::U16 => quote! {
                impl #impl_generics FromNetworkOrder for #enum_name #ty_generics #where_clause {
                    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
                        let value = buffer.read_u16::<BigEndian>()?;
                        match <#enum_name>::try_from(value) {
                            Ok(ct) => {
                                *self = ct;
                                Ok(())
                            }
                            _ => Err(Error::from(ErrorKind::NotFound)),
                        }
                    }
                }
            },
        }
    }

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
}
