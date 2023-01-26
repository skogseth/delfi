use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Datapoint)]
pub fn derive_datapoint(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", ast);
    let name = &ast.ident;

    let expanded = match &ast.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(fields) => {
                    let list = &fields.named;
                    let cols: usize = list.len();
                    let names: Vec<_> = list.iter().map(|e| e.ident.clone().unwrap()).collect();

                    quote! {
                        impl delfi::Datapoint<#cols> for #name {
                            fn record(&self) -> [String; #cols] {
                                [#(self.#names.to_string()),*]
                            }
                        }
                    }
                }
                syn::Fields::Unnamed(fields) => {
                    let list = &fields.unnamed;
                    let cols: usize = list.len();
                    let numbers: Vec<_> = list.iter().enumerate().map(|(i, _)| syn::Index::from(i)).collect();

                    quote! {
                        impl delfi::Datapoint<#cols> for #name {
                            fn record(&self) -> [String; #cols] {
                                [#(self.#numbers.to_string()),*]
                            }
                        }
                    }
                }

                syn::Fields::Unit => panic!("Cannot derive Datapoint for unit struct"),
            }
        }
        _ => unimplemented!(),
    };

    TokenStream::from(expanded)
}
