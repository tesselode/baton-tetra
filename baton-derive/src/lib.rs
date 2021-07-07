use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(ControlKind)]
pub fn control_kind_derive(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(input).unwrap();
	let name = ast.ident;
	match ast.data {
		Data::Enum(data_enum) => {
			let mut control_kind_idents = vec![];
			for variant in data_enum.variants {
				if variant.fields.len() > 0 {
					return syn::Error::new(
						variant.span(),
						"Can only derive ControlKind for enums with fieldless variants",
					)
					.to_compile_error()
					.into();
				}
				control_kind_idents.push(variant.ident.clone());
			}
			(quote! {
				impl baton::traits::ControlKind for #name {
					fn all<'a>() -> &'a [Self] {
						&[#(Self::#control_kind_idents),*]
					}
				}
			})
			.into()
		}
		_ => {
			return syn::Error::new(name.span(), "Can only derive ControlKind for enums")
				.to_compile_error()
				.into();
		}
	}
}
