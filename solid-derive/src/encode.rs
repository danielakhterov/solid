use super::Solidity;
use proc_macro2::{
    Literal,
    TokenStream,
};
use syn::{
    Data,
    DeriveInput,
    Fields,
};

pub(super) fn impl_encode(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    let mut has_name = true;
    let mut name = Literal::string(ident.to_string().as_str());
    for attr in &ast.attrs {
        if let Some(ident) = &attr.path.get_ident() {
            if ident.to_string() == "solid" {
                let attribute = attr.parse_args::<Solidity>().unwrap();
                match attribute.ident.to_string().as_str() {
                    "constructor" => {
                        has_name = false;
                    }

                    "rename" => {
                        name = attribute.name.unwrap();
                    }

                    attribute => panic!("Unsupported key for solidity attribute: {:?}. Supported attribute keys are `rename` and `constructor`", attribute),
                }
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => todo!(),
        },

        _ => todo!(),
    };

    let count = fields.iter().count();

    let field = fields.iter().map(|field| field.ident.clone());

    let ty1 = fields.iter().map(|field| field.ty.clone());
    let ty2 = fields.iter().map(|field| field.ty.clone());
    let ty3 = fields.iter().map(|field| field.ty.clone());
    let ty4 = fields.iter().map(|field| field.ty.clone());

    let encode = quote! {
        fn encode(&self) -> Vec<u8> {
            let name_offset: usize = if #has_name {
                4
            } else {
                0
            };

            let len = self.required_len() + name_offset as u64;

            let mut buf = vec![0u8; len as usize];

            let mut offset = (#count * 32) as usize + name_offset;
            let mut index = 0usize;

            if #has_name {
                let mut selector = solid::Selector::new();
                #(
                    selector = selector.push::<#ty1>();
                )*

                buf[0..4].copy_from_slice(&selector.build(#name));
            }

            #(
                let bytes = self.#field.encode();
                if <#ty2 as solid::encode::Encode>::is_dynamic() {
                    buf[index * 32 + 24..(index + 1) * 32].copy_from_slice(&(offset as u64).to_be_bytes());
                    buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                    offset += bytes.len();
                } else {
                    buf[index * 32 + name_offset..(index + 1) * 32 + name_offset].copy_from_slice(&bytes);
                }
                index += 1;
            )*

            buf
        }
    };

    let field = fields.iter().map(|field| field.ident.clone());

    let required_len = quote! {
        fn required_len(&self) -> u64 {
            let mut len = 0u64;

            #(
                len += if <#ty3 as solid::encode::Encode>::is_dynamic() {
                    32 + self.#field.required_len()
                } else {
                    32
                };
            )*

            len
        }
    };

    let is_dynamic = quote! {
        fn is_dynamic() -> bool {
            true
        }
    };

    let into_type = quote! {
        fn into_type() -> std::borrow::Cow::<'static, str> {
            let mut ty = Vec::new();
            #(
                ty.push(<#ty4 as solid::into_type::IntoType>::into_type());
            )*

            std::borrow::Cow::Owned(format!("({})", ty.join(",")))
        }
    };

    quote! {
        impl #impl_generics solid::encode::Encode for #ident #ty_generics #where_clause {
            #encode

            #required_len

            #is_dynamic
        }

        impl #impl_generics solid::into_type::IntoType for #ident #ty_generics #where_clause {
            #into_type
        }
    }
}
