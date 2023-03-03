// Copyright 2021-2023 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tidext is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

use darling::FromMeta;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, spanned::Spanned};

mod client;
mod runtimes;
mod utils;

#[proc_macro_attribute]
pub fn tidext(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    let msg = "Invalid tidext macro call: expected no attributes, e.g. macro call must be just \
			``#[tidext]`";
    let span = proc_macro2::TokenStream::from(attr).span();
    return syn::Error::new(span, msg).to_compile_error().into();
  }

  let item = syn::parse_macro_input!(item as syn::ItemMod);
  match client::Def::try_from(item) {
    Ok(def) => client::expand(def).into(),
    Err(e) => e.to_compile_error().into(),
  }
}

#[proc_macro_attribute]
pub fn runtimes(attr: TokenStream, item: TokenStream) -> TokenStream {
  let span = proc_macro2::TokenStream::from(attr.clone()).span();
  let attr_args = parse_macro_input!(attr as syn::AttributeArgs);
  let item_mod = parse_macro_input!(item as syn::ItemMod);

  let args = match runtimes::RuntimeMetadataArgs::from_list(&attr_args) {
    Ok(v) => v,
    Err(e) => return e.write_errors().into(),
  };

  match args.runtime_metadata_path.try_into() {
    Ok(runtimes) => runtimes::expand(item_mod, runtimes).into(),
    Err(e) => syn::Error::new(span, e.to_string())
      .to_compile_error()
      .into(),
  }
}
