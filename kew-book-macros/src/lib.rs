use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

type Result<T = (), E = syn::Error> = core::result::Result<T, E>;

#[proc_macro_attribute]
pub fn chapter(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse(input.into()) {
        Ok(v) => v.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn parse(input: TokenStream) -> Result<TokenStream> {
    let fun: syn::ItemFn = syn::parse2(input)?;

    let name = &fun.sig.ident;
    let name_str = name.to_string();
    let test_name = Ident::new(&format!("{name_str}_test"), name.span());
    let ffi_name = Ident::new(&format!("{name_str}_ffi"), name.span());
    let used_name = Ident::new(&format!("{name_str}_used"), name.span());

    let out = quote! {
        #[cfg(not(target_family = "wasm"))]
        #[test]
        fn #test_name() {
            let mut context = kew_book_api::Context::new(
                file!(),
                #name_str
            );
            #name(&mut context);
            context.finish();
        }

        // TODO return the data
        #[cfg(target_family = "wasm")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn #ffi_name(input: wasm_bindgen::JsValue) {
            let mut context = kew_book_api::Context::new(
                input,
                file!(),
                #name_str
            );
            #name(&mut context);
            context.finish();
        }

        // mark it as used
        #[allow(dead_code)]
        #[cfg(not(test))]
        fn #used_name() {
            let _ = #name;
        }

        #fun
    };

    Ok(out)
}
