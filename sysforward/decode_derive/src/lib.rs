use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(DecodeExit)]
pub fn decode_exit_derive(input: TokenStream) -> TokenStream
{
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // The name of the struct
    let name = ast.ident;
    
    // Extract the list of structure fields
    let fields = match ast.data {
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => fields_named.named,
                _ => panic!("Expected named fields in struct"),
            }
        },
        _ => panic!("Expected struct"),
    };

    // Get the retval attribute
    let retval_field = fields.iter()
        .find(|field| field.ident.as_ref().map_or(false, |ident| ident == "retval"))
        .expect("No field retval in the struct");

    // Get the type inside the Option<>
    let inner_type = if let syn::Type::Path(path) = &retval_field.ty {
        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option" {
            let args = &path.path.segments[0].arguments;
            if let syn::PathArguments::AngleBracketed(angle_bracketed) = args {
                if let syn::GenericArgument::Type(ty) = &angle_bracketed.args[0] {
                    Some(ty.clone())
                } else {
                    panic!("Failed to extract inner type of Option<> for retval");
                }
            } else {
                panic!("Failed to extract inner type of Option<> for retval");
            }
        } else {
            panic!("retval is not an Option<>");
        }
    } else {
        panic!("retval is not an Option<>");
    };

    let gen = quote! {
        impl DecodeExit for #name {
            fn decode_exit(&mut self, value: usize, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error>
            {
                self.retval = Some( #inner_type::from(value) );
                Ok(())
            }
        }
    };
    gen.into()
}
