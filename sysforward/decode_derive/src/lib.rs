use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(DecodeExit)]
pub fn decode_exit_derive(input: TokenStream) -> TokenStream
{
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl DecodeExit for #name {
            fn decode_exit(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error>
            {
                self.retval.as_mut().unwrap().decode(pid, operation);
                Ok(())
            }
        }
    };
    gen.into()
}
