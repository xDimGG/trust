use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, Field, Fields, GenericArgument, ItemEnum, PathArguments, Type};

#[proc_macro_attribute]
pub fn message_encoder_decoder(_: TokenStream, input: TokenStream) -> TokenStream {
	let encode_methods = TokenStream2::from(message_encode(input.clone()));
	let decode_methods = TokenStream2::from(message_decode(input.clone()));

	let input = parse_macro_input!(input as ItemEnum);

	let mut structs = Vec::new();
	let mut variants = Vec::new();

	for variant in input.variants {
		if let Fields::Named(fields) = variant.fields {
			let fields = fields.named.iter().map(|e| {
				let ident = &e.ident;
				let ty = &e.ty;
				quote!{ pub #ident: #ty }
			});
			let name = variant.ident;
			structs.push(quote! {
				#[derive(Debug, Clone)]
				pub struct #name {
					#(#fields),*
				}
			});
			variants.push(quote! { #name(#name) })
		} else {
			variants.push(quote! { #variant })
		}
	}

	TokenStream::from(quote! {
		#(#structs)*
		#[derive(Debug, Clone)]
		pub enum Message {
			#(#variants),*
		}
		#encode_methods
		#decode_methods
	})
}

fn message_decode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ItemEnum);
	let mut cases = Vec::new();

	for variant in input.variants {
		let name = variant.ident;
		if name.to_string() == "Custom" {
			continue;
		}

		let doc = variant.attrs.first().unwrap().span().source_text().unwrap();
		if !doc.contains("<-") {
			continue;
		}

		let code: u8 = doc.split_whitespace().skip(1).next().unwrap().parse().unwrap();

		match variant.fields {
			Fields::Named(fields) => {
				let mut field_names = Vec::new();
				let mut field_readers = Vec::new();

				for field in fields.named {
					field_readers.push(field_to_read_method(&field));
					field_names.push(field.ident.unwrap())
				}

				cases.push(quote! { #code => Self::#name(#name { #(#field_names: #field_readers),* }) })
			},
			Fields::Unnamed(fields) => {
				let mut field_readers = Vec::new();

				for field in fields.unnamed {
					field_readers.push(field_to_read_method(&field))
				}

				cases.push(quote! { #code => Self::#name(#(#field_readers),*) })
			},
			Fields::Unit => cases.push(quote! { #code => Self::#name }),
		};
	}

	TokenStream::from(quote! {
		impl<'a> From<Vec<u8>> for Message  {
			fn from(buf: Vec<u8>) -> Self {
				let mut r = Reader::new(&buf);

				match r.read_byte() {
					#(#cases),*,
					code => Self::Custom(code, buf.clone()),
				}
			}
		}
	})
}

fn type_to_read_method(s: &str) -> TokenStream2 {
	match s {
		"bool" => quote! { r.read_bool() },
		"u8" => quote! { r.read_byte() },
		"i8" => quote! { r.read_i8() },
		"u16" => quote! { r.read_u16() },
		"i16" => quote! { r.read_i16() },
		"u32" => quote! { r.read_u32() },
		"i32" => quote! { r.read_i32() },
		"u64" => quote! { r.read_u64() },
		"i64" => quote! { r.read_i64() },
		"f32" => quote! { r.read_f32() },
		"f64" => quote! { r.read_f64() },
		"String" => quote! { r.read_string() },
		"Text" => quote! { r.read_text() },
		"RGB" => quote! { r.read_rgb() },
		"Vector2" => quote! { r.read_vector2() },
		ty => quote! { compile_error!(format!("Unsupported type: {}", #ty)) },
	}
}

fn field_to_read_method(field: &Field) -> TokenStream2 {
	match &field.ty {
		Type::Path(ty) => {
			type_to_read_method(&ty.path.segments.first().unwrap().ident.to_string())
		},
		Type::Array(ty) => {
			if let Type::Path(at) = &*ty.elem {
				let len = &ty.len;
				let ident = &at.path.segments.first().unwrap().ident;
				let method = type_to_read_method(ident.to_string().as_str());
				quote! { {
					let mut buf = [#ident::default(); #len];
					for num in &mut buf {
						*num = #method
					}
					buf
				} }
			} else {
				quote! { compile_error!("Array element is not TypePath") }
			}
		},
		_ => quote! { compile_error!("Field is not TypePath") },
	}
}

fn message_encode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ItemEnum);
	let mut cases = Vec::new();

	for variant in input.variants {
		let name = variant.ident;
		if name.to_string() == "Custom" {
			continue;
		}

		let doc = variant.attrs.first().unwrap().span().source_text().unwrap();
		if !doc.contains("->") {
			continue;
		}

		let code: u8 = doc.split_whitespace().skip(1).next().unwrap().parse().unwrap();

		match variant.fields {
			Fields::Named(fields) => {
				let mut methods = Vec::new();

				for field in fields.named {
					let name = field.ident.as_ref().unwrap();
					methods.push(field_to_write_method(&field, quote! { data.#name }))
				}

				cases.push(quote! {
					Message::#name(data) => {
						let mut w = Writer::new(#code);
						#(#methods);*;
						Ok(w.finalize())
			 		}
				})
			},
			Fields::Unnamed(fields) => {
				let mut args = Vec::new();
				let mut methods = Vec::new();

				for (i, field) in fields.unnamed.iter().enumerate() {
					let i = format_ident!("field{}", i);
					methods.push(field_to_write_method(field, quote! { #i }));
					args.push(quote! { #i })
				}

				cases.push(quote! {
					Message::#name(#(#args),*) => {
						let mut w = Writer::new(#code);
						#(#methods);*;
						Ok(w.finalize())
			 		}
				})
			},
			Fields::Unit => cases.push(quote! {
				Message::#name => Ok(Writer::new(#code).finalize()),
			}),
		};
	}

	TokenStream::from(quote! {
		impl TryFrom<Message> for Vec<u8> {
			type Error = &'static str;

			fn try_from(msg: Message) -> Result<Self, Self::Error> {
				match msg {
					#(#cases)*
					Message::Custom(code, buf) => {
						let mut w = Writer::new(code);
						w.write_bytes(buf.clone());
						Ok(w.finalize())
					}
					_ => Err("Unserializable message. Consider using Message::Custom"),
				}
			}
		}
	})
}

fn type_to_write_method(s: &str, arg: TokenStream2) -> TokenStream2 {
	match s {
		"bool" => quote! { w.write_bool(#arg) },
		"u8" => quote! { w.write_byte(#arg) },
		"i8" => quote! { w.write_i8(#arg) },
		"u16" => quote! { w.write_u16(#arg) },
		"i16" => quote! { w.write_i16(#arg) },
		"u32" => quote! { w.write_u32(#arg) },
		"i32" => quote! { w.write_i32(#arg) },
		"u64" => quote! { w.write_u64(#arg) },
		"i64" => quote! { w.write_i64(#arg) },
		"f32" => quote! { w.write_f32(#arg) },
		"f64" => quote! { w.write_f64(#arg) },
		"String" => quote! { w.write_string(#arg) },
		"Text" => quote! { w.write_text(#arg) },
		"RGB" => quote! { w.write_rgb(#arg) },
		"Vector2" => quote! { w.write_vector2(#arg) },
		e => { dbg!(e); quote! { compile_error!("Unsupported type") } },
	}
}

fn field_to_write_method(field: &Field, name: TokenStream2) -> TokenStream2 {
	match &field.ty {
		Type::Path(ty) => {
			let p_name = &ty.path.segments.first().unwrap().ident.to_string();
			if p_name == "Vec" || p_name == "Option" {
				if let PathArguments::AngleBracketed(ab) = &ty.path.segments.first().unwrap().arguments {
					if let GenericArgument::Type(gt) = &ab.args.first().unwrap() {
						if let Type::Path(gtp) = gt {
							let ident = &gtp.path.segments.first().unwrap().ident.to_string();
							let method = type_to_write_method(ident.as_str(), quote! { x });
							match p_name.as_str() {
								"Vec" => quote! {
									for x in #name {
										#method
									}
								},
								"Option" => quote! {
									if let Some(x) = #name {
										#method
									}
								},
								_ => quote! { compile_error!("Unsupported type") },
							}
						} else { quote! { compile_error!("Unsupported type") } }
					} else { quote! { compile_error!("Unsupported type") } }
				} else { quote! { compile_error!("Unsupported type") } }
			} else {
				type_to_write_method(ty.path.segments.first().unwrap().ident.to_string().as_str(), name)
			}
		},
		Type::Array(ty) => {
			if let Type::Path(at) = &*ty.elem {
				let ident = &at.path.segments.first().unwrap().ident;
				let method = type_to_write_method(ident.to_string().as_str(), quote! { x });
				quote! {
					for x in #name {
						#method
					}
				}
			} else {
				quote! { compile_error!("Array element is not TypePath") }
			}
		},
		_ => quote! { compile_error!("Field is not TypePath") },
	}
}