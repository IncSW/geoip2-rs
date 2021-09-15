use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident,
    ItemStruct, Lit, LitStr, NestedMeta, PathArguments, Type,
};

fn extract_field(field_ident: Ident, ty: &Type) -> proc_macro2::TokenStream {
    match &ty {
        syn::Type::Path(tp) => {
            let segment = &tp.path.segments[0];
            let ident = &segment.ident;
            match ident.to_string().as_str() {
                "Option" => match &segment.arguments {
                    PathArguments::AngleBracketed(ga) => match &ga.args[0] {
                        GenericArgument::Type(ty) => match ty {
                            syn::Type::Path(tp) => {
                                let segment = &tp.path.segments[0];
                                let ident = &segment.ident;
                                match ident.to_string().as_str() {
                                    "u64" => quote! {
                                        self.#field_ident = Some(read_usize(buffer, offset)? as u64)
                                    },
                                    "u32" => quote! {
                                        self.#field_ident = Some(read_usize(buffer, offset)? as u32)
                                    },
                                    "u16" => quote! {
                                        self.#field_ident = Some(read_usize(buffer, offset)? as u16)
                                    },
                                    "f64" => quote! {
                                        self.#field_ident = Some(read_f64(buffer, offset)?)
                                    },
                                    "bool" => quote! {
                                        self.#field_ident = Some(read_bool(buffer, offset)?)
                                    },
                                    "Map" => quote! {
                                        self.#field_ident = Some(read_map(buffer, offset)?)
                                    },
                                    "models" => {
                                        let ident = &tp.path.segments[1].ident;
                                        quote! {
                                            let mut model = models::#ident::default();
                                            model.from_bytes(buffer, offset)?;
                                            self.#field_ident = Some(model);
                                        }
                                    }
                                    "Vec" => match &segment.arguments {
                                        PathArguments::AngleBracketed(ga) => match &ga.args[0] {
                                            GenericArgument::Type(syn::Type::Path(p)) => {
                                                let segment = &p.path.segments[0];
                                                let ident = &segment.ident;
                                                match ident.to_string().as_str() {
                                                    "models" => {
                                                        let ident = &p.path.segments[1].ident;
                                                        quote! {
                                                            let (data_type, size) = read_control(buffer, offset)?;
                                                            self.#field_ident = Some(match data_type {
                                                                DATA_TYPE_SLICE => {
                                                                    let mut array: Vec<models::#ident<'a>> = Vec::with_capacity(size);
                                                                    for _i in 0..size {
                                                                        let mut model = models::#ident::default();
                                                                        model.from_bytes(buffer, offset)?;
                                                                        array.push(model);
                                                                    }
                                                                    array
                                                                }
                                                                DATA_TYPE_POINTER => {
                                                                    let ref mut offset = read_pointer(buffer, offset, size)?;
                                                                    let (data_type, size) = read_control(buffer, offset)?;
                                                                    match data_type {
                                                                        DATA_TYPE_SLICE => {
                                                                            let mut array: Vec<models::#ident<'a>> =
                                                                                Vec::with_capacity(size);
                                                                            for _ in 0..size {
                                                                                let mut model = models::#ident::default();
                                                                                model.from_bytes(buffer, offset)?;
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
                                                    _ => unimplemented!(),
                                                }
                                            }
                                            _ => unimplemented!("{:?}", &ga.args[0]),
                                        },
                                        _ => unimplemented!("{:?}", &segment.arguments),
                                    },
                                    _ => unimplemented!("{:?}", ident),
                                }
                            }
                            syn::Type::Reference(tr) => match tr.elem.as_ref() {
                                syn::Type::Path(tp) => {
                                    let segment = &tp.path.segments[0];
                                    let ident = &segment.ident;
                                    match ident.to_string().as_str() {
                                        "str" => quote! {
                                            self.#field_ident = Some(read_str(buffer, offset)?)
                                        },
                                        _ => unimplemented!("{:?}", ident),
                                    }
                                }
                                _ => unimplemented!("{:?}", tr.elem),
                            },
                            _ => unimplemented!("{:?}", ty),
                        },
                        _ => unimplemented!("{:?}", &ga.args[0]),
                    },
                    _ => unimplemented!("{:?}", &segment.arguments),
                },
                "Vec" => match &segment.arguments {
                    PathArguments::AngleBracketed(ga) => match &ga.args[0] {
                        GenericArgument::Type(ty) => match ty {
                            syn::Type::Reference(tr) => match tr.elem.as_ref() {
                                syn::Type::Path(tp) => {
                                    let segment = &tp.path.segments[0];
                                    let ident = &segment.ident;
                                    match ident.to_string().as_str() {
                                        "str" => quote! {
                                            self.#field_ident = read_array(buffer, offset)?
                                        },
                                        _ => unimplemented!("{:?}", ident),
                                    }
                                }
                                _ => unimplemented!("{:?}", tr.elem),
                            },
                            _ => unimplemented!("{:?}", ty),
                        },
                        _ => unimplemented!("{:?}", &ga.args[0]),
                    },
                    _ => unimplemented!("{:?}", &segment.arguments),
                },
                "u64" => quote! {
                    self.#field_ident = read_usize(buffer, offset)? as u64
                },
                "u32" => quote! {
                    self.#field_ident = read_usize(buffer, offset)? as u32
                },
                "u16" => quote! {
                    self.#field_ident = read_usize(buffer, offset)? as u16
                },
                "Map" => quote! {
                    self.#field_ident = read_map(buffer, offset)?
                },
                _ => unimplemented!("{:?}", ident),
            }
        }
        syn::Type::Reference(tr) => match tr.elem.as_ref() {
            syn::Type::Path(tp) => {
                let segment = &tp.path.segments[0];
                let ident = &segment.ident;
                match ident.to_string().as_str() {
                    "str" => quote! {
                        self.#field_ident = read_str(buffer, offset)?
                    },
                    _ => unimplemented!("{:?}", ident),
                }
            }
            _ => unimplemented!("{:?}", tr.elem),
        },
        _ => unimplemented!("{:?}", ty),
    }
}

fn extract_fields(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let fields = if let syn::Fields::Named(FieldsNamed { named, .. }) = fields {
        named
    } else {
        unimplemented!("{:?}", fields);
    };
    let mut result = Vec::new();
    for field in fields.iter() {
        let field_ident = field.ident.clone().unwrap();
        let mut field_ident_value = format!("{}", field_ident);
        if field_ident_value == "country_type" {
            field_ident_value = "type".into();
        }
        let field_stream = extract_field(field_ident, &field.ty);
        result.push(quote! {
            #field_ident_value => {
                #field_stream
            }
        });
    }
    result
}

#[proc_macro_derive(Decoder)]
pub fn derive_decoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let fields = if let syn::Data::Struct(s) = data {
        extract_fields(&s.fields)
    } else {
        unimplemented!("{:?}", data)
    };

    let output = quote! {
        impl<'a> #ident #generics {
            pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
                let (data_type, size) = read_control(buffer, offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
                    DATA_TYPE_POINTER => {
                        let ref mut offset = read_pointer(buffer, offset, size)?;
                        let (data_type, size) = read_control(buffer, offset)?;
                        match data_type {
                            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
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
                        #(#fields ,)*
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
                unimplemented!("{:?}", item);
            }
        })
        .collect::<Vec<LitStr>>();
    let types_len = types.len();

    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = extract_fields(&input.fields);

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
                let mut result = #ident::default();
                result.from_bytes(self.decoder_buffer, &mut self.get_offset(address)?)?;
                Ok(result)
            }
        }

        impl<'a> #ident #generics {
            pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
                let (data_type, size) = read_control(buffer, offset)?;
                if data_type != DATA_TYPE_MAP {
                    return Err(Error::InvalidDataType(data_type));
                }
                self.from_bytes_map(buffer, offset, size)
            }

            fn from_bytes_map(
                &mut self,
                buffer: &'a [u8],
                offset: &mut usize,
                size: usize,
            ) -> Result<(), Error> {
                for _ in 0..size {
                    match read_str(buffer, offset)? {
                        #(#fields ,)*
                        field => return Err(Error::UnknownField(field.into()))
                    }
                }
                Ok(())
            }
        }
    };
    output.into()
}
