use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Ident, Meta, NestedMeta, Path};

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

fn control_kind_ident(ast: &DeriveInput) -> syn::Result<Ident> {
	let meta = ast
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("control_kind"))
		.ok_or(syn::Error::new(
			ast.ident.span(),
			"Missing control_kind attribute",
		))?
		.parse_meta()?;
	match meta {
		Meta::List(list) => list
			.nested
			.first()
			.map(|nested_meta| match nested_meta {
				NestedMeta::Meta(meta) => meta.path().get_ident(),
				_ => None,
			}).flatten().cloned()
			.ok_or(syn::Error::new(
				list.span(),
				"control_kind attribute should contain a control kind ident",
			)),
		_ => Err(syn::Error::new(
			meta.span(),
			"invalid control_kind attribute",
		)),
	}
}

#[proc_macro_derive(PairKind, attributes(control_kind))]
pub fn pair_kind_derive(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(input).unwrap();
	let name = ast.ident.clone();
	let control_kind_ident = match control_kind_ident(&ast) {
		Ok(ident) => ident,
		Err(err) => return err.to_compile_error().into(),
	};
	match ast.data {
		Data::Enum(data_enum) => todo!(),
		_ => {
			return syn::Error::new(name.span(), "Can only derive PairKind for enums")
				.to_compile_error()
				.into();
		}
	}
}
