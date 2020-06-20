// Copyright 2020, The mxml Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized,braced,Ident,token,Token,LitStr};
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

pub struct Var {
   _brace1: token::Brace,
   _brace2: token::Brace,
   _var: Ident
}
impl Parse for Var {
   fn parse(input: ParseStream) -> Result<Self> {
      let content1;
      let content2;
      Ok(Var {
         _brace1: braced!(content1 in input),
         _brace2: braced!(content2 in content1),
         _var: content2.parse()?
      })
   }
}

pub enum StrOrVar {
   Str(LitStr),
   Var(Var)
}
impl Parse for StrOrVar {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(if input.peek(token::Brace) {
         StrOrVar::Var(input.parse()?)
      } else {
         StrOrVar::Str(input.parse()?)
      })
   }
}

pub struct Attr {
   _attr_prefix: AttrPrefix,
   _key: IdentOrStr,
   _eq: Token![=],
   _val: StrOrVar
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
impl std::fmt::Display for IdentOrAny {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         IdentOrAny::Ident(id) => {
            write!(f, "{}", id.to_string())
         },
         IdentOrAny::Any(_) => {
            write!(f, "?")
         }
      }
   }
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

pub struct FullBody {
   _inner_bracket: Token![>],
   _children: Vec<FME>,
   _post_bracket: Token![<],
   _post_slash: Token![/],
   _post_ident: IdentOrAny
}
impl FullBody {
   fn parse(input: ParseStream, _tag: &str) -> Result<Self> {
      Ok(FullBody {
         _inner_bracket: input.parse()?,
         _children: FME::parse_outer(input)?,
         _post_bracket: input.parse()?,
         _post_slash: input.parse()?,
         _post_ident: input.parse()?
      })
   }
}

pub enum Body {
   SelfClosing(Token![/]),
   FullBody(FullBody)
}
impl Body {
   fn parse(input: ParseStream, tag: &str) -> Result<Self> {
      if input.peek(Token![/]) {
         Ok(Body::SelfClosing(input.parse()?))
      } else {
         Ok(Body::FullBody(FullBody::parse(input, tag)?))
      }
   }
}

pub struct FME {
   _open_bracket: Token![<],
   _name: IdentOrAny,
   _attrs: Vec<Attr>,
   _body: Body,
   _close_bracket: Token![>]
}
impl FME {
   fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
      let mut fmes = Vec::new();
      while !(input.peek(Token![<]) && input.peek2(Token![/])) {
         fmes.push(input.parse()?);
      }
      Ok(fmes)
   }
}
impl Parse for FME {
   fn parse(input: ParseStream) -> Result<Self> {
      let open_bracket: Token![<] = input.parse()?;
      let name: IdentOrAny = input.parse()?;
      let closing_tag = name.to_string();
      Ok(FME {
         _open_bracket: open_bracket,
         _name: name,
         _attrs: Attr::parse_outer(input)?,
         _body: Body::parse(input, &closing_tag)?,
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