extern crate proc_macro;

use proc_macro_error::{abort, proc_macro_error};
use quote::{ToTokens, __private::TokenTree, quote};
use syn::{parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Ident};

use crate::proc_macro::TokenStream;

fn get_attr_idents(attr: &Attribute) -> Result<Vec<Ident>, syn::Error> {
	Ok(attr
		.parse_meta()?
		.to_token_stream()
		.into_iter()
		.find_map(|token| {
			if let TokenTree::Group(group) = token {
				Some(
					group
						.stream()
						.into_iter()
						.filter_map(|token| {
							if let TokenTree::Ident(ident) = token {
								Some(ident)
							} else {
								None
							}
						})
						.collect::<Vec<Ident>>(),
				)
			} else {
				None
			}
		})
		.unwrap_or(vec![]))
}

#[proc_macro_error]
#[proc_macro_derive(ControlKind)]
pub fn control_kind_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	// get the names of each variant in the enum
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
#[proc_macro_derive(PairKind, attributes(control_kind, controls))]
pub fn pair_kind_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	// get the control kind that this pair kind corresponds to
	// this code assumes that the control kind identifier is the
	// first identifier in the first group of the attribute
	let control_kind_attr = input
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("control_kind"))
		.unwrap_or_else(|| {
			abort!(input.span(), "missing control_kind attribute");
		});
	let control_kind_ident = get_attr_idents(control_kind_attr)
		.unwrap_or_else(|_| abort!(control_kind_attr.span(), "invalid control_kind attribute"))
		.drain(0..1)
		.next()
		.unwrap_or_else(|| abort!(control_kind_attr.span(), "missing control kind"));
	// for each enum variant, get the name of the variant
	// plus the four control identifiers in the first group of
	// the controls attribute
	let variants = if let Data::Enum(data_enum) = &input.data {
		data_enum.variants.iter().map(|variant| {
			if variant.fields.len() > 0 {
				abort!(
					variant.fields.span(),
					"PairKind variants must not have fields"
				);
			}
			let controls_attr = variant
				.attrs
				.iter()
				.find(|attr| attr.path.is_ident("controls"))
				.unwrap_or_else(|| {
					abort!(variant.span(), "missing controls attribute");
				});
			let control_idents = get_attr_idents(controls_attr)
				.unwrap_or_else(|_| abort!(controls_attr.span(), "invalid controls attribute"));
			if control_idents.len() != 4 {
				abort!(
					controls_attr.span(),
					"each controls attribute must contain 4 control kinds"
				);
			}
			(&variant.ident, control_idents)
		})
	} else {
		abort!(input.span(), "PairKind can only be derived for enums");
	};
	// unzip the iterator for easier interpolation
	let (variant_idents, control_idents): (Vec<_>, Vec<_>) = variants.unzip();
	// PairKindTrait implementation
	TokenStream::from(quote! {
		impl baton_tetra::pair::PairKindTrait<#control_kind_ident> for #name {
			fn kinds() -> &'static [Self] {
				&[
					#(Self::#variant_idents),*
				]
			}

			fn controls(&self) -> (#control_kind_ident, #control_kind_ident, #control_kind_ident, #control_kind_ident) {
				match self {
					#(
						Self::#variant_idents => (
							#(#control_kind_ident::#control_idents),*
						)
					),*
				}
			}
		}
	})
}
