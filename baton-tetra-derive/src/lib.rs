extern crate proc_macro;

use proc_macro_error::{abort, proc_macro_error};
use quote::{ToTokens, __private::TokenTree, quote};
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

#[proc_macro_error]
#[proc_macro_derive(PairKind, attributes(control_kind))]
pub fn pair_kind_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let control_kind_attr = input
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("control_kind"))
		.unwrap_or_else(|| {
			abort!(input.span(), "missing control_kind attribute");
		});
	let control_kind_ident = control_kind_attr
		.parse_meta()
		.map(|meta| {
			meta.to_token_stream().into_iter().find_map(|token| {
				if let TokenTree::Group(group) = token {
					group.stream().into_iter().find_map(|token| {
						if let TokenTree::Ident(ident) = token {
							Some(ident)
						} else {
							None
						}
					})
				} else {
					None
				}
			})
		})
		.ok()
		.flatten()
		.unwrap_or_else(|| abort!(control_kind_attr.span(), "invalid control_kind attribute"));
	let variant_idents = if let Data::Enum(data_enum) = &input.data {
		data_enum.variants.iter().map(|variant| {
			if variant.fields.len() > 0 {
				abort!(
					variant.fields.span(),
					"PairKind variants must not have fields"
				);
			}
			&variant.ident
		})
	} else {
		abort!(input.span(), "PairKind can only be derived for enums");
	};
	TokenStream::from(quote! {
		impl baton_tetra::pair::PairKindTrait<#control_kind_ident> for #name {
			fn kinds() -> &'static [Self] {
				&[#(Self::#variant_idents),*]
			}
		}
	})
}
