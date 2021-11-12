
extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, ExprField, Fields, Ident, Result, Token};

// import the flatbuffers runtime library
extern crate flatbuffers;

trait LoadFromUFO {

}

// Can we use a proc macro to do the tiresome field:field nonsense?
// Ref https://github.com/dtolnay/syn/issues/516

// https://github.com/dtolnay/proc-macro-workshop

// Copy content from norad object into a new flatbuffer object

// https://docs.rs/syn/1.0.81/syn/parse/struct.ParseBuffer.html#example
// Can't derive debug because Token doesn't support
// struct CreateFromArgs {
//     builder: Ident,
//     ufo_item: ExprField,
//     flat_t: Ident,
// }

// impl std::fmt::Debug for CreateFromArgs {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//         f.debug_struct("CreateFromArgs")
//             .field("builder", &self.builder)
//             .field("ufo_item", &self.ufo_item)
//             .field("flat_t", &self.flat_t)
//             .finish()
//     }
// }

// https://docs.rs/syn/1.0.81/syn/parse/index.html
// impl Parse for CreateFromArgs {
//     fn parse(input: ParseStream) -> Result<Self> {
//         Ok(CreateFromArgs {
//             builder: input.parse()?,
//             _comma1: input.parse()?,
//             ufo_item: input.parse()?,
//             _comma2: input.parse()?,
//             flat_t: input.parse()?,
//         })
//     }
// }

// #[proc_macro]
// pub fn create_from_ufo(tokens: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(tokens as CreateFromArgs);

//     println!("{:#?}", input);
//     TokenStream::from(quote!(
//             5
// ))
// }

// Take 2: slap derives onto the types from flatbuffers
// https://github.com/dtolnay/syn/issues/516
// https://blog.turbo.fish/proc-macro-simple-derive/
#[proc_macro_derive(FromUFO)]
pub fn derive_from_ufo(input_tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_tokens as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;

    println!("FromUFO {:?}", struct_name);
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    for field in fields {
        println!("  {:#?}", field);
    }    

    TokenStream::from(quote! {
        impl #generics std::convert::From for #struct_name #generics {
            // TODO a useful conversion
        }
    })
}