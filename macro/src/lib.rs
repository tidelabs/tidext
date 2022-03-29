use proc_macro::{self, TokenStream};
use syn::spanned::Spanned;

mod expand;
mod parse;
mod utils;

#[proc_macro_attribute]
pub fn tidext(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    let msg = "Invalid pallet macro call: expected no attributes, e.g. macro call must be just \
			``#[tidext]`";
    let span = proc_macro2::TokenStream::from(attr).span();
    return syn::Error::new(span, msg).to_compile_error().into();
  }

  let item = syn::parse_macro_input!(item as syn::ItemMod);
  match parse::Def::try_from(item) {
    Ok(def) => expand::expand(def).into(),
    Err(e) => e.to_compile_error().into(),
  }
}
