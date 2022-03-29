use quote::ToTokens;

pub trait MutItemAttrs {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}

impl MutItemAttrs for syn::Item {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
    match self {
      Self::Const(item) => Some(item.attrs.as_mut()),
      Self::Enum(item) => Some(item.attrs.as_mut()),
      Self::ExternCrate(item) => Some(item.attrs.as_mut()),
      Self::Fn(item) => Some(item.attrs.as_mut()),
      Self::ForeignMod(item) => Some(item.attrs.as_mut()),
      Self::Impl(item) => Some(item.attrs.as_mut()),
      Self::Macro(item) => Some(item.attrs.as_mut()),
      Self::Macro2(item) => Some(item.attrs.as_mut()),
      Self::Mod(item) => Some(item.attrs.as_mut()),
      Self::Static(item) => Some(item.attrs.as_mut()),
      Self::Struct(item) => Some(item.attrs.as_mut()),
      Self::Trait(item) => Some(item.attrs.as_mut()),
      Self::TraitAlias(item) => Some(item.attrs.as_mut()),
      Self::Type(item) => Some(item.attrs.as_mut()),
      Self::Union(item) => Some(item.attrs.as_mut()),
      Self::Use(item) => Some(item.attrs.as_mut()),
      _ => None,
    }
  }
}

impl MutItemAttrs for syn::TraitItem {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
    match self {
      Self::Const(item) => Some(item.attrs.as_mut()),
      Self::Method(item) => Some(item.attrs.as_mut()),
      Self::Type(item) => Some(item.attrs.as_mut()),
      Self::Macro(item) => Some(item.attrs.as_mut()),
      _ => None,
    }
  }
}

impl MutItemAttrs for Vec<syn::Attribute> {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
    Some(self)
  }
}

impl MutItemAttrs for syn::ItemMod {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
    Some(&mut self.attrs)
  }
}

impl MutItemAttrs for syn::ImplItemMethod {
  fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
    Some(&mut self.attrs)
  }
}

pub fn take_item_tidext_attrs<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Vec<Attr>>
where
  Attr: syn::parse::Parse,
{
  let mut pallet_attrs = Vec::new();

  while let Some(attr) = take_first_item_tidext_attr(item)? {
    pallet_attrs.push(attr)
  }

  Ok(pallet_attrs)
}

pub fn take_first_item_tidext_attr<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Option<Attr>>
where
  Attr: syn::parse::Parse,
{
  let attrs = if let Some(attrs) = item.mut_item_attrs() {
    attrs
  } else {
    return Ok(None);
  };

  if let Some(index) = attrs.iter().position(|attr| {
    attr
      .path
      .segments
      .first()
      .map_or(false, |segment| segment.ident == "tidext")
  }) {
    let pallet_attr = attrs.remove(index);
    Ok(Some(syn::parse2(pallet_attr.into_token_stream())?))
  } else {
    Ok(None)
  }
}
