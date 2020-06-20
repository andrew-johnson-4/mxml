// Copyright 2020, The mxml Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized,Ident,token,Token};
use syn::punctuated::Punctuated;

/*
mixin!(Tooltip(message),
  <? +"data-toggle"="tooltip" +"data-placement"="top" +title={{message}}/>
);
*/

pub struct Mixin {
   name: Ident,
   _paren_token: token::Paren,
   _args: Punctuated<Ident, Token![,]>,
   _comma: Token![,],
}
impl Parse for Mixin {
   fn parse(input: ParseStream) -> Result<Self> {
      let content;
      Ok(Mixin {
         name: input.parse()?,
         _paren_token: parenthesized!(content in input),
         _args: content.parse_terminated(Ident::parse)?,
         _comma: input.parse()?
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
