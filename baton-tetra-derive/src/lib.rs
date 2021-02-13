extern crate proc_macro;

use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

use crate::proc_macro::TokenStream;

#[proc_macro_error]
#[proc_macro_derive(ControlKind)]
pub fn control_kind_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let variant_idents = if let Data::Enum(data_enum) = &input.data {
		data_enum.variants.iter().map(|variant| {
			if variant.fields.len() > 0 {
				abort!(
					variant.fields.span(),
					"ControlKind variants must not have fields"
				);
			}
			&variant.ident
		})
	} else {
		abort!(input.span(), "ControlKind can only be derived for enums");
	};
	TokenStream::from(quote! {
		impl baton_tetra::control::ControlKindTrait for #name {
			fn kinds() -> &'static [Self] {
				&[#(Self::#variant_idents),*]
			}
		}
	})
}
