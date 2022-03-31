// Copyright 2021-2022 Semantic Network Ltd.
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

use proc_macro::{self, TokenStream};
use syn::spanned::Spanned;

mod expand;
mod parse;
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
  match parse::Def::try_from(item) {
    Ok(def) => expand::expand(def).into(),
    Err(e) => e.to_compile_error().into(),
  }
}
