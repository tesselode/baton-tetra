use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

extern crate proc_macro;

#[proc_macro_derive(ControlKind)]
pub fn control_kind_derive(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(input).unwrap();
	let name = ast.ident;
	let control_kind_idents: Vec<Ident> = match ast.data {
		Data::Enum(data_enum) => data_enum
			.variants
			.iter()
			.map(|variant| {
				if variant.fields.len() > 0 {
					panic!("Can only derive ControlKind for fieldless enums")
				}
				variant.ident.clone()
			})
			.collect(),
		_ => panic!("Can only derive ControlKind for enums"),
	};
	let output = quote! {
		impl baton::traits::ControlKind for #name {
			fn all<'a>() -> &'a [Self] {
				&[#(Self::#control_kind_idents),*]
			}
		}
	};
	output.into()
}
