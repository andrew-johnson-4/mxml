// Copyright 2020, The mxml Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{parenthesized,braced,Ident,token,Token,LitStr};
use syn::punctuated::Punctuated;
use proc_macro2::{Span,Literal};

pub enum AttrPrefix {
   Match(Token![~]),
   Add(Token![+]),
}
impl AttrPrefix {
   fn span(&self) -> Span {
      match self {
         AttrPrefix::Match(t) => { t.span }
         AttrPrefix::Add(t) => { t.span }
      }
   }
   fn is_match(&self) -> bool {
      match self {
         AttrPrefix::Match(_) => { true }
         _ => { false }
      }
   }
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
   var: Ident
}
impl Parse for Var {
   fn parse(input: ParseStream) -> Result<Self> {
      let content1;
      let content2;
      Ok(Var {
         _brace1: braced!(content1 in input),
         _brace2: braced!(content2 in content1),
         var: content2.parse()?
      })
   }
}
impl ToTokens for Var {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      self.var.to_tokens(tokens);
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
impl ToTokens for StrOrVar {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      match self {
         StrOrVar::Str(s) => {
            quote!(
               #s.to_string()
            ).to_tokens(tokens);
         },
         StrOrVar::Var(v) => {
            quote!(
               #v.to_string()
            ).to_tokens(tokens);
         }
      }
   }
}

pub struct Attr {
   attr_prefix: AttrPrefix,
   key: IdentOrStr,
   _eq: Token![=],
   val: StrOrVar
}
impl ToTokens for Attr {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let span = self.attr_prefix.span();
      let key = Literal::string(&self.key.to_string());
      let ref val = self.val;
      match self.attr_prefix {
         AttrPrefix::Match(_) => {
            quote_spanned!(span=>
               mxml_dep::Match::HasAttributeValue(#key.to_string(),#val),
            ).to_tokens(tokens);
         },
         _ => {
            quote_spanned!(span=>
               mxml_dep::Edit::AddAttribute(#key.to_string(),#val),
            ).to_tokens(tokens);
         }
      }
   }
}
impl Parse for Attr {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(Attr {
         attr_prefix: input.parse()?,
         key: input.parse()?,
         _eq: input.parse()?,
         val: input.parse()?
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
impl std::fmt::Display for IdentOrStr {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         IdentOrStr::Ident(id) => {
            write!(f, "{}", id.to_string())
         },
         IdentOrStr::Str(s) => {
            write!(f, "{}", s.value())
         }
      }
   }
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
impl IdentOrAny {
   fn span(&self) -> Span {
      match self {
         IdentOrAny::Ident(i) => { i.span() },
         IdentOrAny::Any(i) => { i.span },
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
impl ToTokens for IdentOrAny {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let span = self.span();
      match self {
         IdentOrAny::Ident(i) => {
            let tag = Literal::string(&i.to_string());
            quote_spanned!(span=>
               mxml_dep::Match::HasTag(#tag.to_string()),
            ).to_tokens(tokens);
         },
         _ => {}
      }
   }
}

pub struct FullBody {
   _inner_bracket: Token![>],
   children: Vec<FME>,
   _post_bracket: Token![<],
   _post_slash: Token![/],
   _post_ident: IdentOrAny
}
impl FullBody {
   fn parse(input: ParseStream, tag: &str) -> Result<Self> {
      Ok(FullBody {
         _inner_bracket: input.parse()?,
         children: FME::parse_outer(input)?,
         _post_bracket: input.parse()?,
         _post_slash: input.parse()?,
         _post_ident: {
            let name: IdentOrAny = input.parse()?;
            if name.to_string() != tag {
              let msg = format!("Expected </{}> found </{}>", tag, name);
              let r = Error::new(name.span(), msg);
              return Err(r)
            }
            name
         }
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
impl ToTokens for Body {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      if let Body::FullBody(b) = self {
         let ref cs = b.children;
         quote!(
           #(#cs)*
         ).to_tokens(tokens);
      }
   }
}

pub struct FME {
   open_bracket: Token![<],
   name: IdentOrAny,
   attrs: Vec<Attr>,
   body: Body,
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
         open_bracket: open_bracket,
         name: name,
         attrs: Attr::parse_outer(input)?,
         body: Body::parse(input, &closing_tag)?,
         _close_bracket: input.parse()?
      })
   }
}
impl ToTokens for FME {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let span = self.open_bracket.span;
      let ref name = self.name;
      let match_attrs: Vec<&Attr> = self.attrs.iter().filter(|a| a.attr_prefix.is_match()).collect();
      let edit_attrs: Vec<&Attr> = self.attrs.iter().filter(|a| !a.attr_prefix.is_match()).collect();
      let ref body = self.body;
      quote_spanned!(span=>
         (
           mxml_dep::FindElement{find:vec![]},
           mxml_dep::MatchElement{when:vec![#name #(#match_attrs)*]},
           mxml_dep::EditElement{edit:vec![#(#edit_attrs)*]}
         ),
         #body
      ).to_tokens(tokens);
   }
}

pub struct Mixin {
   name: Ident,
   _paren_token: token::Paren,
   args: Punctuated<Ident, Token![,]>,
   _comma: Token![,],
   fme: FME
}
impl Parse for Mixin {
   fn parse(input: ParseStream) -> Result<Self> {
      let content;
      Ok(Mixin {
         name: input.parse()?,
         _paren_token: parenthesized!(content in input),
         args: content.parse_terminated(Ident::parse)?,
         _comma: input.parse()?,
         fme: input.parse()?
      })
   }
}
impl ToTokens for Mixin {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let ref name = self.name;
      let args: Vec<&Ident> = self.args.iter().collect();
      let ref fme = self.fme;
      let span = name.span();
 
      quote_spanned!(span=>
         fn #name(#(#args: &str,)*) -> mxml_dep::FindMatchEditElement {
            mxml_dep::FindMatchEditElement {
               fme: vec![
                 #fme
               ]
            }
         }
      ).to_tokens(tokens);
   }
}
