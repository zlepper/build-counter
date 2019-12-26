#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::export::TokenStream2;
use syn::{Data, DeriveInput, Ident, Type};

#[proc_macro_derive(InjectedResource)]
pub fn injected_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let expanded = quote! {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for #struct_name {
            type Error = ();
            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, ()> {
                let state = request.guard::<::rocket::State<#struct_name>>()?;

                ::rocket::request::Outcome::Success(state.to_owned())
            }
        }
    };

    expanded.into()
}

struct Dep {
    ty: Type,
    name: Ident,
}

#[proc_macro_derive(Dependency)]
pub fn dependency(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("Dependency derive has to be done on a struct"),
    };

    let fs = fields.iter().map(|e| {
        let ty = e.ty.clone();

        Dep {
            ty,
            name: e.ident.clone().unwrap(),
        }
    });

    let finders: Vec<TokenStream2> = fs
        .clone()
        .map(|f| {
            let name = f.name;
            let ty = f.ty;

            quote! {
                let #name = request.guard::<#ty>()?;
            }
        })
        .collect();
    let fields: Vec<Ident> = fs.map(|f| f.name).collect();

    let result = quote! {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for #struct_name {
            type Error = ();
            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, ()> {
                #(#finders)*

                let instance = #struct_name {
                    #(#fields)*
                };

                ::rocket::request::Outcome::Success(instance)
            }
        }
    };

    result.into()
}

#[proc_macro]
pub fn static_resources(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{:?}", input);

    let result = quote! {#input};

    result.into()
}
