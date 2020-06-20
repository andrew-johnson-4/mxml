// Copyright 2020, The mxml Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident};

/*
mixin!(Tooltip(message),
  <? +"data-toggle"="tooltip" +"data-placement"="top" +title={{message}}/>
);
*/

pub struct Mixin {
   name: Ident
}
impl Parse for Mixin {
   fn parse(input: ParseStream) -> Result<Self> {
      let name: Ident = input.parse()?;

      Ok(Mixin {
         name: name
      })
   }
}
impl ToTokens for Mixin {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let ref name = self.name;
      let span = name.span();
 
      quote_spanned!(span=>
         fn #name() {}
      ).to_tokens(tokens);
   }
}
