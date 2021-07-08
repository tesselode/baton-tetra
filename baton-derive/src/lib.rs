use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Ident, Meta, NestedMeta, Variant};

extern crate proc_macro;

#[proc_macro_derive(ControlKind)]
pub fn control_kind_derive(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(input).unwrap();
	let name = ast.ident;
	let data_enum = match ast.data {
		Data::Enum(data_enum) => data_enum,
		_ => {
			return syn::Error::new(name.span(), "can only derive ControlKind for enums")
				.to_compile_error()
				.into()
		}
	};
	let mut control_kind_idents = vec![];
	for variant in data_enum.variants {
		if variant.fields.len() > 0 {
			return syn::Error::new(
				variant.span(),
				"can only derive ControlKind for enums with fieldless variants",
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

#[proc_macro_derive(PairKind, attributes(control_kind, controls))]
pub fn pair_kind_derive(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(input).unwrap();
	let name = ast.ident.clone();
	let control_kind_enum_ident = match control_kind_enum_ident(&ast) {
		Ok(ident) => ident,
		Err(err) => return err.to_compile_error().into(),
	};
	let data_enum = match ast.data {
		Data::Enum(data_enum) => data_enum,
		_ => {
			return syn::Error::new(name.span(), "can only derive PairKind for enums")
				.to_compile_error()
				.into()
		}
	};
	let mut pair_kind_idents = vec![];
	let mut control_kind_idents = vec![];
	for variant in &data_enum.variants {
		if variant.fields.len() > 0 {
			return syn::Error::new(
				variant.span(),
				"can only derive PairKind for enums with fieldless variants",
			)
			.to_compile_error()
			.into();
		}
		pair_kind_idents.push(variant.ident.clone());
		let idents = match control_kind_idents_for_pair_kind_variant(variant) {
			Ok(idents) => idents,
			Err(err) => return err.to_compile_error().into(),
		};
		control_kind_idents.push(idents);
	}
	(quote! {
		impl baton::traits::PairKind<ControlKind> for PairKind {
			fn all<'a>() -> &'a [Self] {
				&[#(Self::#pair_kind_idents),*]
			}

			fn controls(&self) -> (#control_kind_enum_ident, #control_kind_enum_ident, #control_kind_enum_ident, #control_kind_enum_ident) {
				match self {
					#(Self::#pair_kind_idents => (
						#(#control_kind_enum_ident::#control_kind_idents),*
					)),*
				}
			}
		}
	})
	.into()
}

fn control_kind_enum_ident(ast: &DeriveInput) -> syn::Result<Ident> {
	let meta = ast
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("control_kind"))
		.ok_or(syn::Error::new(
			ast.ident.span(),
			"missing control_kind attribute",
		))?
		.parse_meta()?;
	let list = match meta {
		Meta::List(list) => list,
		_ => {
			return Err(syn::Error::new(
				meta.span(),
				"invalid control_kind attribute",
			))
		}
	};
	list.nested
		.first()
		.map(|nested_meta| match nested_meta {
			NestedMeta::Meta(meta) => meta.path().get_ident(),
			_ => None,
		})
		.flatten()
		.cloned()
		.ok_or(syn::Error::new(
			list.span(),
			"control_kind attribute should contain a control kind identifier",
		))
}

fn control_kind_idents_for_pair_kind_variant(variant: &Variant) -> syn::Result<Vec<Ident>> {
	let meta = variant
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("controls"))
		.ok_or(syn::Error::new(
			variant.span(),
			"missing controls attribute",
		))?
		.parse_meta()?;
	let list = match meta {
		Meta::List(list) => list,
		_ => return Err(syn::Error::new(meta.span(), "invalid controls attribute")),
	};
	let mut idents = vec![];
	for nested_meta in list.nested.iter().take(4) {
		if let NestedMeta::Meta(meta) = nested_meta {
			idents.push(meta.path().get_ident().cloned().ok_or(syn::Error::new(
				nested_meta.span(),
				"control should be an identifier",
			))?);
		} else {
			Err(syn::Error::new(
				nested_meta.span(),
				"invalid controls attribute",
			))?
		}
	}
	if idents.len() < 4 {
		return Err(syn::Error::new(
			list.span(),
			"each pair kind should have 4 controls (left, right, up, down)",
		));
	}
	Ok(idents)
}
