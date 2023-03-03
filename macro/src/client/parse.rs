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

use crate::utils;
use syn::spanned::Spanned;

mod keyword {
  syn::custom_keyword!(tidext);
  syn::custom_keyword!(client);
  syn::custom_keyword!(subxt);
  syn::custom_keyword!(custom);
}

mod keyword_fn {
  syn::custom_keyword!(pallet);
  syn::custom_keyword!(substitute_fn);
  syn::custom_keyword!(substitute_params);
  syn::custom_keyword!(rpc);
  syn::custom_keyword!(consts);
}

/// Parse for one of the following:
/// * `#[tidext::pallet = "tidefi"]`
/// * `#[tidext::substitute_fn = "batch"]`
/// * `#[tidext::rpc = "health"]`
/// * `#[tidext::consts = "market_maker_fee_amount"]`
#[derive(Debug)]
pub enum FnAttr {
  Pallet(syn::Ident, proc_macro2::Span),
  Rename(syn::Ident, proc_macro2::Span),
  Params(syn::ExprTuple, proc_macro2::Span),
  Rpc(syn::Ident, proc_macro2::Span),
  Const(syn::Ident, proc_macro2::Span),
}

impl FnAttr {
  fn attr_span(&self) -> proc_macro2::Span {
    match self {
      Self::Pallet(_, span)
      | Self::Rename(_, span)
      | Self::Params(_, span)
      | Self::Rpc(_, span)
      | Self::Const(_, span) => *span,
    }
  }
}

impl syn::parse::Parse for FnAttr {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    input.parse::<syn::Token![#]>()?;
    let attr_span = input.span();
    let content;
    syn::bracketed!(content in input);
    content.parse::<keyword::tidext>()?;
    content.parse::<syn::Token![::]>()?;

    let lookahead = content.lookahead1();
    if lookahead.peek(keyword_fn::pallet) {
      content.parse::<keyword_fn::pallet>()?;
      content.parse::<syn::Token![=]>()?;
      let renamed_prefix = content.parse::<syn::LitStr>()?;

      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Pallet(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::substitute_fn) {
      content.parse::<keyword_fn::substitute_fn>()?;
      content.parse::<syn::Token![=]>()?;
      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Rename(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::substitute_params) {
      content.parse::<keyword_fn::substitute_params>()?;
      content.parse::<syn::Token![=]>()?;

      let expr_tuple = content.parse::<syn::ExprTuple>()?;
      Ok(Self::Params(expr_tuple, attr_span))
    } else if lookahead.peek(keyword_fn::rpc) {
      content.parse::<keyword_fn::rpc>()?;
      content.parse::<syn::Token![=]>()?;
      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Rpc(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::consts) {
      content.parse::<keyword_fn::consts>()?;
      content.parse::<syn::Token![=]>()?;

      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Const(new_ident, attr_span))
    } else {
      Err(lookahead.error())
    }
  }
}

struct FnAttrInfo {
  pallet: syn::Ident,
  params: Option<syn::ExprTuple>,
  rename: Option<syn::Ident>,
  is_rpc: Option<syn::Ident>,
  is_const: Option<syn::Ident>,
}

impl FnAttrInfo {
  fn from_attrs(attrs: Vec<FnAttr>, item_span: proc_macro2::Span) -> syn::Result<Self> {
    let mut pallet = None;
    let mut rename = None;
    let mut is_rpc = None;
    let mut is_const = None;
    let mut params = None;

    for attr in attrs {
      match attr {
        FnAttr::Pallet(ident, ..) if pallet.is_none() => pallet = Some(ident),
        FnAttr::Rename(ident, ..) if rename.is_none() => rename = Some(ident),
        FnAttr::Params(ident, ..) if params.is_none() => params = Some(ident),
        FnAttr::Rpc(found_rpc, ..) if is_rpc.is_none() => is_rpc = Some(found_rpc),
        FnAttr::Const(found_const, ..) if is_const.is_none() => is_const = Some(found_const),
        attr => {
          return Err(syn::Error::new(
            attr.attr_span(),
            "Invalid attribute: Duplicate attribute",
          ))
        }
      }
    }

    Ok(FnAttrInfo {
      is_rpc,
      is_const,
      rename,
      params,
      pallet: pallet.ok_or_else(|| syn::Error::new(item_span, "Missing `#[tidext::pallet]`"))?,
    })
  }
}

pub struct ClientDef {
  pub inner: syn::ItemStruct,
}

impl ClientDef {
  pub fn try_from(_attr_span: proc_macro2::Span, item: &mut syn::Item) -> syn::Result<Self> {
    let item = if let syn::Item::Struct(item) = item {
      item
    } else {
      return Err(syn::Error::new(
        item.span(),
        "Invalid tidext::client, expect struct type.",
      ));
    };

    Ok(ClientDef {
      inner: item.clone(),
    })
  }
}

pub struct CustomDef {
  pub inner: syn::ItemImpl,
}

impl CustomDef {
  pub fn try_from(_attr_span: proc_macro2::Span, item: &mut syn::Item) -> syn::Result<Self> {
    let item = if let syn::Item::Impl(item) = item {
      item
    } else {
      return Err(syn::Error::new(
        item.span(),
        "Invalid tidext::custom, expect impl type.",
      ));
    };

    Ok(CustomDef {
      inner: item.clone(),
    })
  }
}

#[derive(Debug)]
pub struct FnItem {
  pub item: syn::TraitItemMethod,
  pub rename_as: Option<syn::Ident>,
  pub pallet: syn::Ident,
  pub is_rpc: Option<syn::Ident>,
  pub is_const: Option<syn::Ident>,
  pub docs: Vec<syn::Lit>,
  pub params_overwrite: Option<syn::ExprTuple>,
}

impl FnItem {
  pub fn function_name(&self) -> syn::Ident {
    self
      .rename_as
      .as_ref()
      .unwrap_or(&self.item.sig.ident)
      .clone()
  }

  pub fn input_typed(&self) -> Vec<syn::PatType> {
    self
      .item
      .sig
      .inputs
      .iter()
      .filter_map(|input| {
        if let syn::FnArg::Typed(input_typed) = input {
          Some(input_typed.clone())
        } else {
          None
        }
      })
      .collect()
  }
}

#[derive(Debug)]
pub struct CallDef {
  pub vis: syn::Visibility,
  pub ident: syn::Ident,
  pub rename_as: Option<syn::Ident>,
  pub items: Vec<FnItem>,
}

impl CallDef {
  pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
    let item = if let syn::Item::Trait(item) = item.clone() {
      item
    } else {
      return Err(syn::Error::new(
        item.span(),
        "Invalid tidext::subxt, expect trait type.",
      ));
    };

    let vis = item.clone().vis;

    let all_trait_items = item
      .items
      .iter()
      .filter_map(|method| {
        if let syn::TraitItem::Method(method) = method {
          let attrs: Vec<FnAttr> =
            utils::take_item_tidext_attrs(&mut method.attrs.clone()).unwrap();
          let FnAttrInfo {
            pallet,
            rename,
            is_const,
            is_rpc,
            params,
          } = FnAttrInfo::from_attrs(attrs, method.span()).unwrap();

          let docs = get_doc_literals(&method.attrs.clone());

          Some(FnItem {
            pallet,
            item: method.clone(),
            rename_as: rename,
            is_rpc,
            is_const,
            docs,
            params_overwrite: params,
          })
        } else {
          None
        }
      })
      .collect();

    Ok(CallDef {
      vis,
      ident: item.ident,
      rename_as: None,
      items: all_trait_items,
    })
  }
}

#[derive(Debug)]
enum DefAttr {
  Subxt(proc_macro2::Span),
  Client(proc_macro2::Span),
  Custom(proc_macro2::Span),
}

impl DefAttr {
  fn span(&self) -> proc_macro2::Span {
    match self {
      Self::Subxt(span) => *span,
      Self::Client(span) => *span,
      Self::Custom(span) => *span,
    }
  }
}

impl syn::parse::Parse for DefAttr {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    input.parse::<syn::Token![#]>()?;
    let content;
    syn::bracketed!(content in input);
    content.parse::<keyword::tidext>()?;
    content.parse::<syn::Token![::]>()?;

    let lookahead = content.lookahead1();
    if lookahead.peek(keyword::subxt) {
      Ok(DefAttr::Subxt(content.parse::<keyword::subxt>()?.span()))
    } else if lookahead.peek(keyword::client) {
      Ok(DefAttr::Client(content.parse::<keyword::client>()?.span()))
    } else if lookahead.peek(keyword::custom) {
      Ok(DefAttr::Custom(content.parse::<keyword::custom>()?.span()))
    } else {
      Err(lookahead.error())
    }
  }
}

pub struct Def {
  pub item: syn::ItemMod,
  pub calls: CallDef,
  pub client: ClientDef,
  pub custom_def: Option<CustomDef>,
}

impl Def {
  pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
    let item_span = item.span();
    let items = &mut item
      .content
      .as_mut()
      .ok_or_else(|| {
        let msg = "Invalid tidext definition, expected mod to be inlined.";
        syn::Error::new(item_span, msg)
      })?
      .1;

    let mut calls = None;
    let mut client = None;
    let mut custom_def = None;

    for item in items.iter_mut() {
      let pallet_attr: Option<DefAttr> = utils::take_first_item_tidext_attr(item)?;

      match pallet_attr {
        Some(DefAttr::Subxt(_span)) if calls.is_none() => calls = Some(CallDef::try_from(item)?),
        Some(DefAttr::Client(span)) if client.is_none() => {
          client = Some(ClientDef::try_from(span, item)?)
        }
        Some(DefAttr::Custom(span)) if custom_def.is_none() => {
          custom_def = Some(CustomDef::try_from(span, item)?)
        }
        Some(attr) => {
          let msg = "Invalid duplicated attribute";
          return Err(syn::Error::new(attr.span(), msg));
        }
        None => (),
      }
    }

    let def = Def {
      item,
      custom_def,
      calls: calls.ok_or_else(|| syn::Error::new(item_span, "Missing `#[tidext::subxt]`"))?,
      client: client.ok_or_else(|| syn::Error::new(item_span, "Missing `#[tidext::client]`"))?,
    };

    Ok(def)
  }
}

fn get_doc_literals(attrs: &[syn::Attribute]) -> Vec<syn::Lit> {
  attrs
    .iter()
    .filter_map(|attr| {
      if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
        if meta.path.get_ident().map_or(false, |ident| ident == "doc") {
          Some(meta.lit)
        } else {
          None
        }
      } else {
        None
      }
    })
    .collect()
}
