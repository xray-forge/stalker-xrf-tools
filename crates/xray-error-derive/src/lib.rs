extern crate proc_macro;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, LitStr, Meta};

#[proc_macro_derive(ErrorConstructors, attributes(constructor))]
pub fn error_constructors_derive(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input as DeriveInput);
  let name: &Ident = &input.ident;

  let data = match &input.data {
    Data::Enum(DataEnum { variants, .. }) => variants,
    _ => panic!("#[derive(ErrorConstructors)] can only be used on enums"),
  };

  let constructors = data.iter().filter_map(|variant| {
    let variant_name: &Ident = &variant.ident;
    let mut custom_fn_name: Option<String> = None;

    for attr in &variant.attrs {
      if attr.path().is_ident("constructor") {
        match attr.meta {
          Meta::Path(_) => {
            custom_fn_name = Some(format!(
              "new_{}_error",
              variant_name.to_string().to_case(Case::Snake)
            ))
          }
          Meta::List(_) => match attr.parse_args::<LitStr>() {
            Ok(literal) => custom_fn_name = Some(literal.value()),
            Err(error) => panic!("#[constructor(name)] macros failed: {}", error),
          },
          Meta::NameValue(_) => {
            panic!("#[constructor(name)] macros failed: unexpected name=value syntax")
          }
        }
      }
    }

    if let Some(custom_fn_name) = custom_fn_name {
      let custom_fn_name: Ident = Ident::new(&custom_fn_name, input.span());

      Some(quote! {
          pub fn #custom_fn_name<T>(message: T) -> Self
          where
              T: Into<String>,
          {
              Self::#variant_name { message: message.into() }
          }
      })
    } else {
      None
    }
  });

  let gen = quote! {
      impl #name {
          #(#constructors)*
      }
  };

  gen.into()
}
