#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::export::TokenStream2;
use syn::{Data, DeriveInput, Ident, Item, Type};
//
//#[proc_macro_derive(InjectedResource)]
//pub fn injected_resource(input: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(input as DeriveInput);
//
//    let struct_name = input.ident;
//
//    let expanded = quote! {
//        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for #struct_name {
//            type Error = ();
//            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, ()> {
//                let state = request.guard::<::rocket::State<#struct_name>>()?;
//
//                ::rocket::request::Outcome::Success(state.to_owned())
//            }
//        }
//    };
//
//    expanded.into()
//}
//
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
                let #name = req.app_data::<#ty>().expect("Dependency of type #ty did not exist").clone();
            }
        })
        .collect();
    let fields: Vec<Ident> = fs.map(|f| f.name).collect();

    let result = quote! {
        impl ::actix_web::FromRequest for #struct_name {
            type Error = ::actix_web::Error;
            type Future = ::futures::future::Ready<Result<Self, ::actix_web::Error>>;
            type Config = ();

            fn from_request(req: &::actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                #(#finders)*

                let instance = #struct_name {
                    #(#fields)*
                };

                ::futures::future::ok(instance)
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn dynamic_dependency(attr: TokenStream, item: TokenStream) -> TokenStream {
    let actual_struct = format_ident!("{}", attr.to_string());

    let item_to_parse = item.clone();
    let trait_def = parse_macro_input!(item_to_parse as Item);

    if let Item::Trait(td) = trait_def {
        let name = td.ident;

        let def = TokenStream2::from(item);

        let expanded = quote! {
            #def

            impl ::actix_web::FromRequest for Box<dyn #name> {
                type Error = ::actix_web::Error;
                type Future = ::futures::future::Ready<Result<Self, ::actix_web::Error>>;
                type Config = ();

                fn from_request(req: &::actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                    #actual_struct::from_request(&req, payload).map_ok(|dep| Box::new(dep))
                }
            }

        };

        expanded.into()
    } else {
        panic!("dynamic_dependency macro should be used on a trait");
    }
}
//
//#[proc_macro]
//pub fn static_resources(input: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(input as DeriveInput);
//
//    println!("{:?}", input);
//
//    let result = quote! {#input};
//
//    result.into()
//}
