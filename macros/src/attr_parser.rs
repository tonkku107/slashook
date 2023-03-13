// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use devise::Spanned;
use proc_macro2::{TokenStream, TokenTree, Span};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
  Token, bracketed, braced,
  Result, Error, ExprAssign, Expr, Ident,
  token::{Bracket, Brace},
  parse2, parse::{Parse, ParseStream, Peek}
};

#[derive(Debug)]
pub(crate) struct Attributes(Vec<(Ident, Expr)>);

#[derive(Debug)]
enum Item {
  Attributes(Attributes, Span),
  Expr(Expr)
}

#[derive(Debug)]
struct AttributeArray(Vec<Item>);

impl Parse for Attributes {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut attrs = Vec::new();

    while !input.is_empty() {
      let mut segment;

      if input.peek3(Bracket) || input.peek3(Brace) {
        // If we have a bracket or brace coming, we need to do some more parsing.
        // Check the name of the value and determine the struct type to be used. Put the name into the segment when done.
        let name: Ident = input.parse()?;
        let struct_type = get_struct_type(&name);
        segment = name.to_token_stream();

        if input.peek2(Bracket) {
          segment.extend(parse_until(input, Bracket)?);

          // Parse the tokens within brackets using a separate parser and put the resulting converted tokens into the segment.
          let bracket_segment;
          bracketed!(bracket_segment in input);
          let attr_arr: AttributeArray = bracket_segment.parse()?;
          segment.extend(attr_arr.to_tokens(struct_type)?);
        } else {
          segment.extend(parse_until(input, Brace)?);
          let span = input.span();

          // Parse the tokens within braces using a this parser and put the resulting converted tokens into the segment.
          let brace_segment;
          braced!(brace_segment in input);
          let item = Item::Attributes(brace_segment.parse()?, span);
          segment.extend(item.to_tokens(&struct_type)?);
        }

        // Error if there is no comma after the brackets or braces
        let lookahead = input.lookahead1();
        if !lookahead.peek(Token![,]) && !input.is_empty() {
          return Err(lookahead.error());
        }
      } else {
        // Otherwise we can just parse what we have no problem.
        segment = parse_until(input, Token!(,))?;
      }

      // Parse what we just obtained as an assignment expression, aka `something = something_else`
      let value: ExprAssign = parse2(segment)?;

      let name;
      // The left expr seems to be a path based on debug logging, even if it's a single identifier...
      if let Expr::Path(expr) = *value.left {
        // Get the identifier from the path
        name = expr.path.get_ident().ok_or_else(|| Error::new(expr.path.span(), "Expected an identifier"))?.clone();

        // We set func in the macro, so error if someone somehow decides to add it for some reason
        if name == "func" {
          return Err(Error::new(name.span(), "Cannot set `func` here. Your handler function goes below this attribute macro."));
        }
      } else {
        return Err(Error::new(value.left.span(), "Expected identifier"));
      }

      let expr = *value.right;
      attrs.push((name, expr));

      // Parse the comma but we don't really care about it
      if input.peek(Token!(,)) {
        let _: Token![,] = input.parse()?;
      }
    }

    Ok(Attributes(attrs))
  }
}

impl ToTokens for Attributes {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.append_separated(self.0.iter().map(|(name, value)| quote! {#name: #value.try_into().unwrap()}), quote! {,});
  }
}

impl Parse for AttributeArray {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut vec = Vec::new();

    while !input.is_empty() {
      if input.peek(Brace) {
        // If there's a brace we need to parse the contents
        let span = input.span();

        let content;
        braced!(content in input);

        // Parse the insides of the braces with the original parser
        let attrs: Attributes = content.parse()?;
        vec.push(Item::Attributes(attrs, span));

        // Error if there is no comma after the braces
        let lookahead = input.lookahead1();
        if !lookahead.peek(Token![,]) && !input.is_empty() {
          return Err(lookahead.error());
        }
      } else {
        // Otherwise we just take whatever expression we got
        let segment = parse_until(input, Token![,])?;
        vec.push(Item::Expr(parse2(segment)?));
      }

      // Parse the comma but we don't really care about it
      if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
      }
    }

    Ok(AttributeArray(vec))
  }
}

impl Item {
  fn to_tokens(&self, struct_type: &Option<TokenStream>) -> Result<TokenStream> {
    Ok(match self {
      Self::Attributes(attrs, span) => {
        if struct_type.is_none() {
          return Err(Error::new(*span, "Didn't expect an object for this field"));
        }

        quote! {
          #struct_type {
            #attrs,
            ..Default::default()
          }
        }
      },
      Self::Expr(expr) => quote! { #expr }
    })
  }
}

impl AttributeArray {
  fn to_tokens(&self, struct_type: Option<TokenStream>) -> Result<TokenStream> {
    let items = self.0.iter().map(|item| {
      item.to_tokens(&struct_type)
    }).collect::<Result<Vec<TokenStream>>>()?;

    Ok(quote! {
      vec![
        #( #items.try_into().unwrap() ),*
      ]
    })
  }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> Result<TokenStream> {
  let mut tokens = TokenStream::new();
  while !input.is_empty() && !input.peek(end) {
    let next: TokenTree = input.parse()?;
    tokens.extend(Some(next));
  }
  Ok(tokens)
}

fn get_struct_type(name: &Ident) -> Option<TokenStream> {
  match name.to_string().as_str() {
    "options" => Some(quote! { slashook::structs::interactions::ApplicationCommandOption }),
    "subcommand_groups" => Some(quote! { slashook::commands::SubcommandGroup }),
    "subcommands" => Some(quote! { slashook::commands::Subcommand }),
    "choices" => Some(quote! { slashook::structs::interactions::ApplicationCommandOptionChoice }),
    _ => None
  }
}
