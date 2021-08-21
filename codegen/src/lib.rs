// TODO: rewrite macro
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, FieldsNamed, GenericArgument, ItemStruct, Lit,
    LitStr, NestedMeta, PathArguments,
};

#[proc_macro_derive(Decoder)]
pub fn derive_decoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let mut map_fields = Vec::new();
    match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                for field in named.iter() {
                    let name_ident = field.ident.clone().unwrap();
                    let name = format!("{}", name_ident);
                    let decode = match &field.ty {
                        syn::Type::Path(p) => {
                            let segment = &p.path.segments[0];
                            match segment.ident.to_string().as_ref() {
                                "Option" => match &segment.arguments {
                                    PathArguments::AngleBracketed(p) => match &p.args[0] {
                                        GenericArgument::Type(ty) => match ty {
                                            syn::Type::Path(p) => {
                                                let segment = &p.path.segments[0];
                                                match segment.ident.to_string().as_ref() {
                                                    "u64" => quote! {
                                                        self.#name_ident = Some(read_usize(buffer, offset)? as u64)
                                                    },
                                                    "u32" => quote! {
                                                        self.#name_ident = Some(read_usize(buffer, offset)? as u32)
                                                    },
                                                    "u16" => quote! {
                                                        self.#name_ident = Some(read_usize(buffer, offset)? as u16)
                                                    },
                                                    "f64" => quote! {
                                                        self.#name_ident = Some(read_f64(buffer, offset)?)
                                                    },
                                                    "bool" => quote! {
                                                        self.#name_ident = Some(read_bool(buffer, offset)?)
                                                    },
                                                    "Map" => quote! {
                                                        self.#name_ident = Some(read_map(buffer, offset)?)
                                                    },
                                                    _ => unimplemented!(),
                                                }
                                            }
                                            syn::Type::Reference(p) => match p.elem.as_ref() {
                                                syn::Type::Path(p) => {
                                                    let segment = &p.path.segments[0];
                                                    match segment.ident.to_string().as_ref() {
                                                        "str" => quote! {
                                                            self.#name_ident = Some(read_str(buffer, offset)?)
                                                        },
                                                        _ => unimplemented!(),
                                                    }
                                                }
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                },
                                "Vec" => match &segment.arguments {
                                    PathArguments::AngleBracketed(p) => match &p.args[0] {
                                        GenericArgument::Type(ty) => match ty {
                                            syn::Type::Reference(p) => match p.elem.as_ref() {
                                                syn::Type::Path(p) => {
                                                    let segment = &p.path.segments[0];
                                                    match segment.ident.to_string().as_ref() {
                                                        "str" => quote! {
                                                            self.#name_ident = read_array(buffer, offset)?
                                                        },
                                                        _ => unimplemented!(),
                                                    }
                                                }
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                },
                                "u64" => quote! {
                                    self.#name_ident = read_usize(buffer, offset)? as u64
                                },
                                "u32" => quote! {
                                    self.#name_ident = read_usize(buffer, offset)? as u32
                                },
                                "u16" => quote! {
                                    self.#name_ident = read_usize(buffer, offset)? as u16
                                },
                                "f64" => quote! {
                                    self.#name_ident = read_f64(buffer, offset)?
                                },
                                "bool" => quote! {
                                    self.#name_ident = read_bool(buffer, offset)?
                                },
                                "Map" => quote! {
                                    self.#name_ident = read_map(buffer, offset)?
                                },
                                _ => unimplemented!(),
                            }
                        }
                        syn::Type::Reference(p) => match p.elem.as_ref() {
                            syn::Type::Path(p) => {
                                let segment = &p.path.segments[0];
                                match segment.ident.to_string().as_ref() {
                                    "str" => quote! {
                                        self.#name_ident = read_str(buffer, offset)?
                                    },
                                    _ => unimplemented!(),
                                }
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    };
                    map_fields.push(quote! {
                        #name => #decode,
                    });
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let output = quote! {
        impl<'a> #ident #generics {
            pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
                let (data_type, size) = read_control(buffer, offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
                    DATA_TYPE_POINTER => {
                        let mut offset = read_pointer(buffer, offset, size)?;
                        let (data_type, size) = read_control(buffer, &mut offset)?;
                        match data_type {
                            DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                            _ => return Err(Error::InvalidDataType(data_type)),
                        }
                    }
                    _ => return Err(Error::InvalidDataType(data_type)),
                }
            }

            fn from_bytes_map(
                &mut self,
                buffer: &'a [u8],
                offset: &mut usize,
                size: usize,
            ) -> Result<(), Error> {
                for _ in 0..size {
                    match read_str(buffer, offset)? {
                        #(#map_fields)*
                        field => return Err(Error::UnknownField(field.into()))
                    }
                }
                Ok(())
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn reader(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let types = parse_macro_input!(metadata as AttributeArgs)
        .iter()
        .map(|item| {
            if let NestedMeta::Lit(Lit::Str(lit)) = item {
                lit.clone()
            } else {
                unreachable!(-100);
            }
        })
        .collect::<Vec<LitStr>>();
    let types_len = types.len();

    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;
    let generics = &input.generics;
    let mut fields = Vec::new();
    if let syn::Fields::Named(FieldsNamed { named, .. }) = &input.fields {
        for field in named.iter() {
            let name_ident = field.ident.clone().unwrap();
            let name = format!("{}", name_ident);
            let decode_stream = match &field.ty {
                syn::Type::Path(p) => {
                    let segment = &p.path.segments[0];
                    let ident = &segment.ident;
                    match ident.to_string().as_ref() {
                        "Option" => match &segment.arguments {
                            PathArguments::AngleBracketed(p) => match &p.args[0] {
                                GenericArgument::Type(ty) => match ty {
                                    syn::Type::Path(p) => {
                                        let segment = &p.path.segments[0];
                                        let ident = &segment.ident;
                                        match ident.to_string().as_ref() {
                                            "u32" => quote! {
                                                #name => result.#name_ident = Some(read_usize(self.decoder_buffer, &mut offset)? as u32)
                                            },
                                            "bool" => quote! {
                                                #name => result.#name_ident = Some(read_bool(self.decoder_buffer, &mut offset)?)
                                            },
                                            "models" => {
                                                let ident = &p.path.segments[1].ident;
                                                quote! {
                                                    #name => {
                                                        let mut model = models::#ident::default();
                                                        model.from_bytes(self.decoder_buffer, &mut offset)?;
                                                        result.#name_ident = Some(model);
                                                    }
                                                }
                                            }
                                            "Vec" => match &segment.arguments {
                                                PathArguments::AngleBracketed(p) => {
                                                    match &p.args[0] {
                                                        GenericArgument::Type(ty) => match ty {
                                                            syn::Type::Path(p) => {
                                                                let segment = &p.path.segments[0];
                                                                let ident = &segment.ident;
                                                                match ident.to_string().as_ref() {
                                                                    "models" => {
                                                                        let ident =
                                                                            &p.path.segments[1]
                                                                                .ident;
                                                                        quote! {
                                                                            #name => {
                                                                                let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
                                                                                result.#name_ident = Some(match data_type {
                                                                                    DATA_TYPE_SLICE => {
                                                                                        let mut array: Vec<models::#ident<'a>> = Vec::with_capacity(size);
                                                                                        for _i in 0..size {
                                                                                            let mut model = models::#ident::default();
                                                                                            model.from_bytes(self.decoder_buffer, &mut offset)?;
                                                                                            array.push(model);
                                                                                        }
                                                                                        array
                                                                                    }
                                                                                    DATA_TYPE_POINTER => {
                                                                                        let mut offset = read_pointer(self.decoder_buffer, &mut offset, size)?;
                                                                                        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
                                                                                        match data_type {
                                                                                            DATA_TYPE_SLICE => {
                                                                                                let mut array: Vec<models::#ident<'a>> =
                                                                                                    Vec::with_capacity(size);
                                                                                                for _ in 0..size {
                                                                                                    let mut model = models::#ident::default();
                                                                                                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                                                                                                    array.push(model);
                                                                                                }
                                                                                                array
                                                                                            }
                                                                                            _ => return Err(Error::InvalidDataType(data_type)),
                                                                                        }
                                                                                    }
                                                                                    _ => return Err(Error::InvalidDataType(data_type)),
                                                                                })
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => unreachable!(),
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        },
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                _ => unreachable!(),
                                            },
                                            _ => unreachable!(ident.to_string()),
                                        }
                                    }
                                    syn::Type::Reference(p) => match p.elem.as_ref() {
                                        syn::Type::Path(p) => {
                                            let ident = &p.path.segments[0].ident;
                                            match ident.to_string().as_ref() {
                                                "str" => quote! {
                                                    #name => result.#name_ident = Some(read_str(self.decoder_buffer, &mut offset)?)
                                                },
                                                _ => unreachable!(ident.to_string()),
                                            }
                                        }
                                        _ => unreachable!(2),
                                    },
                                    _ => unreachable!(3),
                                },
                                _ => unreachable!(4),
                            },
                            _ => unreachable!(5),
                        },
                        "u32" => quote! {
                            #name => result.#name_ident = read_usize(self.decoder_buffer, &mut offset)? as u32
                        },
                        "bool" => quote! {
                            #name => result.#name_ident = read_bool(self.decoder_buffer, &mut offset)?
                        },
                        _ => unreachable!(ident.to_string()),
                    }
                }
                _ => unreachable!(7),
            };
            fields.push(decode_stream);
        }
    } else {
        unreachable!(8);
    }

    let output = quote! {
        #input

        impl<'a> Reader<'a, #ident #generics> {
            pub fn from_bytes(buffer: &[u8]) -> Result<Reader<#ident>, Error> {
                const types: [&'static str; #types_len] = [#(#types ,)*];
                let reader = Reader::from_bytes_raw(buffer)?;
                if !types.contains(&reader.metadata.database_type) {
                    return Err(Error::InvalidDatabaseType(
                        reader.metadata.database_type.into(),
                    ));
                }
                Ok(reader)
            }

            pub fn lookup(&self, address: IpAddr) -> Result<#ident, Error> {
                let mut offset = self.get_offset(address)?;
                let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
                if data_type != DATA_TYPE_MAP {
                    return Err(Error::InvalidDataType(data_type));
                }
                let mut result = #ident::default();
                for _ in 0..size {
                    match read_str(self.decoder_buffer, &mut offset)? {
                        #(#fields ,)*
                        field => return Err(Error::UnknownField(field.into()))
                    }
                }
                Ok(result)
            }
        }
    };
    output.into()
}
