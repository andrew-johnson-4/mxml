// Copyright 2020, The mxml Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized,Ident,token,Token,LitStr};
use syn::punctuated::Punctuated;

pub enum AttrPrefix {
   Match(Token![~]),
   Add(Token![+]),
}
impl Parse for AttrPrefix {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(if input.peek(Token![~]) {
         AttrPrefix::Match(input.parse()?)
      } else {
         AttrPrefix::Add(input.parse()?)
      })
   }
}

pub struct Attr {
   _attr_prefix: AttrPrefix,
   _key: IdentOrStr,
   _eq: Token![=],
   _val: LitStr
}
impl Parse for Attr {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(Attr {
         _attr_prefix: input.parse()?,
         _key: input.parse()?,
         _eq: input.parse()?,
         _val: input.parse()?
      })
   }
}
impl Attr {
   fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
      let mut many = Vec::new();
      while input.peek(Token![~]) || input.peek(Token![+]) {
         many.push(input.parse()?);
      }
      Ok(many)
   }
}

pub enum IdentOrStr {
   Ident(Ident),
   Str(LitStr)
}
impl Parse for IdentOrStr {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(if input.peek(LitStr) {
         IdentOrStr::Str(input.parse()?)
      } else {
         IdentOrStr::Ident(input.parse()?)
      })
   }
}

pub enum IdentOrAny {
   Ident(Ident),
   Any(Token![?])
}
impl Parse for IdentOrAny {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(if input.peek(Token![?]) {
         IdentOrAny::Any(input.parse()?)
      } else {
         IdentOrAny::Ident(input.parse()?)
      })
   }
}

pub struct FME {
   _open_bracket: Token![<],
   _name: IdentOrAny,
   _attrs: Vec<Attr>,
   _close_bracket: Token![>]
}
impl Parse for FME {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(FME {
         _open_bracket: input.parse()?,
         _name: input.parse()?,
         _attrs: Attr::parse_outer(input)?,
         _close_bracket: input.parse()?
      })
   }
}

pub struct Mixin {
   name: Ident,
   _paren_token: token::Paren,
   _args: Punctuated<Ident, Token![,]>,
   _comma: Token![,],
   _fme: FME
}
impl Parse for Mixin {
   fn parse(input: ParseStream) -> Result<Self> {
      let content;
      Ok(Mixin {
         name: input.parse()?,
         _paren_token: parenthesized!(content in input),
         _args: content.parse_terminated(Ident::parse)?,
         _comma: input.parse()?,
         _fme: input.parse()?
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
