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

use crate::parse::Def;
use quote::{format_ident, ToTokens};

pub fn expand(mut def: Def) -> proc_macro2::TokenStream {
  let calls = expand_calls(&mut def);

  quote::quote!(
     #calls
  )
}

pub fn expand_calls(def: &mut Def) -> proc_macro2::TokenStream {
  let client_mod = &def.item.ident;
  let client_mod_vis = &def.item.vis;
  let client_struct_name = &def.client.inner.ident;
  let client_builder_struct_name = format_ident!("{}Builder", &def.client.inner.ident);
  let client_struct = &def.client.inner.to_token_stream();
  let custom_impl = if let Some(custom_impl) = &def.custom_def {
    let custom_impl_token = custom_impl.inner.clone().into_token_stream();
    quote::quote!(
       #custom_impl_token
    )
  } else {
    quote::quote!()
  };

  let all_functions = def.calls.items.iter().map(|attr| {

    let trait_function_name = &attr.item.sig.ident;
    let function_name = &attr.function_name();
    let args = &attr.input_typed();
    let function_and_wait_for_finalized_success =
      format_ident!("{}_and_wait_for_finalized_success", trait_function_name);
    let function_extrinsic = format_ident!("{}_extrinsic", trait_function_name);
    let function_doc = &attr.docs;

    let all_params: Vec<syn::Ident> = args
      .iter()
      .filter_map(|pat_type| {
        if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
          Some(pat_ident.ident)
        } else {
          None
        }
      })
      .collect();

    let all_params_token : proc_macro2::TokenStream = if let Some(params) = &attr.params_overwrite {
      params.elems.to_token_stream()
    } else {
      quote::quote!(#(#all_params,)*)
    };

    let pallet_name = &attr.pallet;

    if let Some(rpc_call) = &attr.is_rpc {
      let result_type = &attr.item.sig.output;
      let request_ident = format!("{}_{}", pallet_name, rpc_call);

      let params = quote::quote!(rpc_params![#(#all_params),*]);

      quote::quote!(
        #( #[doc = #function_doc] )*
        pub async fn #trait_function_name(
          &self,
          #(#args,)*
        ) #result_type {
          self
            .runtime()
            .rpc()
            .request(#request_ident, #params)
            .await
            .map_err(Into::into)
        }
      )
    } else if let Some(const_call) = &attr.is_const {
      let result_type = &attr.item.sig.output;

      quote::quote!(
        #( #[doc = #function_doc] )*
        pub async fn #trait_function_name(
          &self,
          #(#args,)*
        ) #result_type {
          with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .constants()
              .at(&current_runtime.constants().#pallet_name().#const_call())
              .map_err(Into::into)
            }
          }
        }
      )
    } else {
      quote::quote!(
        #( #[doc = #function_doc] )*
        pub async fn #trait_function_name(
          &self,
          #(#args,)*
        ) -> Result<(), Error> {
          with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .tx()
              .sign_and_submit_default(&current_runtime.tx().#pallet_name().#function_name(#all_params_token), self.signer()?)
              .await?
            }
          };

          Ok(())
        }

        #( #[doc = #function_doc] )*
        ///
        #[doc = "This function wait for finalization on chain and may take up to 30 seconds to complete"]
        pub async fn #function_and_wait_for_finalized_success(
          &self,
          #(#args,)*
        ) -> Result<(), Error> {

          with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .tx()
              .sign_and_submit_then_watch_default(&current_runtime.tx().#pallet_name().#function_name(#all_params_token), self.signer()?)
              .await?
            }
          }
          .wait_for_finalized_success()
          .await?;

          Ok(())
        }

        #[doc = "Generate a signed extrinsic"]
        ///
        #( #[doc = #function_doc] )*
        pub async fn #function_extrinsic(
          &self,
          #(#args,)*
        ) -> Result<String, Error> {
          let extrinsic = with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .tx()
              .create_signed(&current_runtime.tx().#pallet_name().#function_name(#all_params_token), self.signer()?, Default::default())
              .await?
            }
          };
          Ok(format!("0x{}", hex::encode(extrinsic.encoded()).as_str()))
        }
      )
    }
  });

  quote::quote!(
    #client_mod_vis mod #client_mod {
      use super::*;
      use jsonrpsee::ws_client::WsClientBuilder;
      use subxt::OnlineClient;
      use std::time::Duration;

      /// Tidechain [`Client`] builder
      #[derive(Clone)]
      pub struct #client_builder_struct_name {
        pub rpc_url: String,
        pub signer: Option<TidefiKeyring>,
        pub runtime: Option<TidefiRuntime>,
      }

      impl Default for #client_builder_struct_name {
        fn default() -> Self {
          Self {
            rpc_url: "ws://127.0.0.1:9944".into(),
            signer: None,
            runtime: None,
          }
        }
      }

      impl ClientBuilder {
        /// Creates a new [`ClientBuilder`]
        pub fn new() -> Self {
          trace!("Creating new ClientBuilder");
          Default::default()
        }

        /// Set the tidechain RPC address (optional)
        pub fn set_url<P: Into<String>>(mut self, url: P) -> Self {
          self.rpc_url = url.into();
          self
        }

        /// Set the tidechain signer (optional)
        pub fn set_signer(mut self, signer: TidefiKeyring) -> Self {
          self.signer = Some(signer);
          self
        }

        /// Set the runtime
        pub fn set_runtime(mut self, runtime: TidefiRuntime) -> Self {
          self.runtime = Some(runtime);
          self
        }

        /// Initialize a new [`Client`]
        pub async fn build(self) -> Result<Client, Error> {
          let ws_client = WsClientBuilder::default()
            .max_notifs_per_subscription(4096)
            .connection_timeout(Duration::from_secs(5))
            .build(&self.rpc_url)
            .await
            .map_err(|err| Error::Other(err.to_string()))?;

          let client = Arc::new(
            subxt::OnlineClient::<TidechainConfig>::from_rpc_client(ws_client)
              .await?
          );

          #[cfg(feature = "tidechain-native")]
          let default_runtime = TidefiRuntime::Tidechain(Arc::new(TidechainRuntime::default()));

          #[cfg(feature = "lagoon-native")]
          let default_runtime = TidefiRuntime::Lagoon(Arc::new(LagoonRuntime::default()));

          let runtime_type = match self.runtime {
            Some(runtime) => runtime,
            None => {
              if self.rpc_url == "wss://rpc.tidefi.io:443" {
                #[cfg(feature = "lagoon-native")]
                let default_runtime = TidefiRuntime::Lagoon(Arc::new(LagoonRuntime::default()));

                #[cfg(feature = "tidechain-native")]
                let default_runtime = TidefiRuntime::Tidechain(Arc::new(TidechainRuntime::default()));

                default_runtime
              } else {
                default_runtime
              }
            }
          };

          log::debug!("Tidext Runtime: {}", runtime_type);

          return Ok(Client {
            signer: self.signer,
            runtime_type,
            client,
          });
        }
      }

      #[derive(Clone)]
      #client_struct

      impl #client_struct_name {
        #(#all_functions)*
      }

      #custom_impl
    }
  )
}
