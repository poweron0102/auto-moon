use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(TileTextureLoadGet)]
pub fn tile_texture_load_get(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let variants = if let syn::Data::Enum(data_enum) = data {
        data_enum.variants
    } else {
        panic!("TileTextureLoadGet can only be applied to enums");
    };

    let variant_idents: Vec<Ident> = variants.into_iter().map(|variant| variant.ident).collect();
    let variant_strings: Vec<String> = variant_idents.iter().map(|ident| ident.to_string()).collect();

    let statics = variant_idents.iter().map(|ident| {
        let static_ident = Ident::new(&format!("{}_TEXTURE", ident), ident.span());
        quote! {
            static mut #static_ident: Option<Texture2D> = None;
        }
    });

    let loads = variant_idents.iter().zip(variant_strings.iter()).map(|(ident, string)| {
        let static_ident = Ident::new(&format!("{}_TEXTURE", ident), ident.span());
        quote! {
            #static_ident = Some(load_texture(&*format!("assets/tiles/{}.png", #string)).await.unwrap());
        }
    });

    let gets = variant_idents.iter().map(|ident| {
        let static_ident = Ident::new(&format!("{}_TEXTURE", ident), ident.span());
        quote! {
        Self::#ident => #static_ident.as_ref().unwrap(),
    }
    });

    let expanded = quote! {
    #(#statics)*

    impl #ident {
        pub async unsafe fn load() {
            #(#loads)*
        }

        pub fn get(&self) -> &'static Texture2D {
            unsafe {
                match self {
                    #(#gets)*
                }
            }
        }
    }
};

    TokenStream::from(expanded)
}