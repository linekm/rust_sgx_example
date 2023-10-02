use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(CMov)]
pub fn derive_cmov(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let cnd_select = impl_cnd_select(&input.data);
    let cnd_assign = impl_cnd_assign(&input.data);
    let cnd_swap = impl_cnd_swap(&input.data);

    let expanded = quote! {
        impl #impl_generics CMov for #name #ty_generics #where_clause {
            #[inline]
            fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
                #cnd_select
            }

            #[inline]
            fn cnd_assign(&mut self, other: &Self, choice: bool) {
                #cnd_assign
            }

            #[inline]
            fn cnd_swap(a: &mut Self, b: &mut Self, choice: bool) {
                #cnd_swap
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_cnd_select(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() =>
                        #name: <_ as CMov>::cnd_select(&a.#name, &b.#name, choice)
                    }
                });
                quote! {
                    Self { #(#recurse),* }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! { f.span() =>
                        <_ as CMov>::cnd_select(&a.#index, &b.#index, choice)
                    }
                });
                quote! {
                    Self(#(#recurse),*)
                }
            }
            Fields::Unit => {
                quote! { Self }
            }
        },
        _ => abort_call_site!("only struct is supported"),
    }
}

fn impl_cnd_assign(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() =>
                        self.#name.cnd_assign(&other.#name, choice);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! { f.span() =>
                        self.#index.cnd_assign(&other.#index, choice);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        _ => abort_call_site!("only struct is supported"),
    }
}

fn impl_cnd_swap(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() =>
                        <_ as CMov>::cnd_swap(&mut a.#name, &mut b.#name, choice);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! { f.span() =>
                        <_ as CMov>::cnd_swap(&mut a.#index, &mut b.#index, choice);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        _ => abort_call_site!("only struct is supported"),
    }
}
