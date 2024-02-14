use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, Field, Fields, ItemEnum, Type};

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
		pub enum Message<'a> {
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
		if name.to_string() == "Unknown" {
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
					field_readers.push(message_decode_type(&field));
					field_names.push(field.ident.unwrap())
				}

				cases.push(quote! { #code => Self::#name(#name { #(#field_names: #field_readers),* }) })
			},
			Fields::Unnamed(fields) => {
				let mut field_readers = Vec::new();

				for field in fields.unnamed {
					field_readers.push(message_decode_type(&field))
				}

				cases.push(quote! { #code => Self::#name(#(#field_readers),*) })
			},
			Fields::Unit => cases.push(quote! { #code => Self::#name }),
		};
	}

	TokenStream::from(quote! {
		impl<'a> From<&'a [u8]> for Message<'a>  {
			fn from(buf: &'a [u8]) -> Self {
				let mut mr = Reader::new(buf);

				match mr.read_byte() {
					#(#cases),*,
					code => Self::Unknown(code, &buf[1..]),
				}
			}
		}
	})
}

fn message_decode_type(field: &Field) -> TokenStream2 {
	match &field.ty {
		Type::Path(ty) => {
			match ty.path.segments.first().unwrap().ident.to_string().as_str() {
				"bool" => quote! { mr.read_bool() },
				"u8" => quote! { mr.read_byte() },
				"i8" => quote! { mr.read_i8() },
				"u16" => quote! { mr.read_u16() },
				"i16" => quote! { mr.read_i16() },
				"u32" => quote! { mr.read_u32() },
				"i32" => quote! { mr.read_i32() },
				"u64" => quote! { mr.read_u64() },
				"i64" => quote! { mr.read_i64() },
				"String" => quote! { mr.read_string() },
				"RGB" => quote! { mr.read_rgb() },
				ty => quote! { compile_error!("Unsupported type: {}", #ty) },
			}
		},
		Type::Array(ty) => {
			if let Type::Path(at) = &*ty.elem {
				let len = &ty.len;
				match at.path.segments.first().unwrap().ident.to_string().as_str() {
					"u16" => quote! { {
						let mut buf = [0u16; #len];
						for num in &mut buf {
							*num = u16::from_le_bytes(mr.read_bytes(2).try_into().unwrap())
						}
						buf
					} },
					ty => quote! { compile_error!("Unsupported array type: {}", #ty) },
				}
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
		if name.to_string() == "Unknown" {
			continue;
		}

		let doc = variant.attrs.first().unwrap().span().source_text().unwrap();
		if !doc.contains("->") {
			continue;
		}

		let code: u8 = doc.split_whitespace().skip(1).next().unwrap().parse().unwrap();

		match variant.fields {
			Fields::Named(fields) => {
				let mut field_readers = Vec::new();

				for field in fields.named {
					let name = field.ident.as_ref().unwrap();
					field_readers.push(message_encode_type(&field, quote! { data.#name }))
				}

				cases.push(quote! { Message::#name(data) => Ok(Writer::new(#code)#(#field_readers)*.finalize()) })
			},
			Fields::Unnamed(fields) => {
				let mut field_names = Vec::new();
				let mut field_readers = Vec::new();

				for (i, field) in fields.unnamed.iter().enumerate() {
					let i = format_ident!("field{}", i);
					field_readers.push(message_encode_type(field, quote! { #i }));
					field_names.push(quote! { #i })
				}

				cases.push(quote! { Message::#name(#(#field_names),*) => Ok(Writer::new(#code)#(#field_readers)*.finalize()) })
			},
			Fields::Unit => cases.push(quote! { Message::#name => Ok(Writer::new(#code).finalize()) }),
		};
	}

	TokenStream::from(quote! {
		impl<'a> TryFrom<Message<'a>> for Vec<u8> {
			type Error = &'static str;

			fn try_from(msg: Message) -> Result<Self, Self::Error> {
				match msg {
					#(#cases),*,
					Message::Unknown(code, buf) => Ok(Writer::new(code).write_bytes(buf).finalize()),
					_ => Err("Unserializable message. Consider using Message::Unknown"),
				}
			}
		}
	})
}

fn message_encode_type(field: &Field, name: TokenStream2) -> TokenStream2 {
	if let Type::Path(ty) = &field.ty {
		match ty.path.segments.first().unwrap().ident.to_string().as_str() {
			"bool" => quote! { .write_bool(#name) },
			"u8" => quote! { .write_byte(#name) },
			"i8" => quote! { .write_i8(#name) },
			"u16" => quote! { .write_u16(#name) },
			"i16" => quote! { .write_i16(#name) },
			"u32" => quote! { .write_u32(#name) },
			"i32" => quote! { .write_i32(#name) },
			"u64" => quote! { .write_u64(#name) },
			"i64" => quote! { .write_i64(#name) },
			"String" => quote! { .write_string(#name) },
			"Text" => quote! { .write_text(#name) },
			"RGB" => quote! { .write_rgb(#name) },
			ty => quote! { compile_error!("Unknown type: {}", #ty) },
		}
	} else {
		quote! { compile_error!("Field is not TypePath") }
	}
}