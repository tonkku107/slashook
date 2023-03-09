// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use devise::Spanned;
use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{Token, ExprAssign, Expr, Ident, bracketed, braced,
  parse::{Parse, ParseStream, Peek}
};

#[derive(Debug)]
pub(crate) struct Attributes(Vec<(Ident, Expr)>);
#[derive(Debug)]
struct AttributeArray(Vec<Attributes>);

impl Parse for Attributes {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut attrs = Vec::new();

    while !input.is_empty() {
      let mut segment;

      if input.peek3(syn::token::Bracket) {
        // If we have a bracket coming, we need to do some more parsing.
        // Check the name of the value and determine the struct type to be used. Put the name into the segment when done.
        let name: Ident = input.parse()?;
        let struct_type = get_struct_type(&name)?;
        segment = name.to_token_stream();
        segment.extend(parse_until(input, syn::token::Bracket)?);

        // Parse the tokens within brackets using a separate parser and put the resulting converted tokens into the segment.
        let bracket_segment;
        bracketed!(bracket_segment in input);
        let attr_arr: AttributeArray = bracket_segment.parse()?;
        segment.extend(attr_arr.to_tokens(struct_type));

        // Just to make sure everything is parsed, if there's anything here it'll probably result in a parse error later
        segment.extend(parse_until(input, Token!(,))?);
      } else {
        // Otherwise we can just parse what we have no problem.
        segment = parse_until(input, Token!(,))?;
      }

      // Parse what we just obtained as an assignment expression, aka `something = something_else`
      let value: ExprAssign = syn::parse2(segment)?;

      let name;
      // The left expr seems to be a path based on debug logging, even if it's a single identifier...
      if let Expr::Path(expr) = *value.left {
        // Get the identifier from the path
        name = expr.path.get_ident().ok_or_else(|| syn::Error::new(expr.path.span(), "Expected an identifier"))?.clone();

        // We set func in the macro, so error if someone somehow decides to add it for some reason
        if name == "func" {
          return Err(syn::Error::new(name.span(), "Cannot set `func` here. Your handler function goes below this attribute macro."));
        }
      } else {
        return Err(syn::Error::new(value.left.span(), "Expected identifier"));
      }

      let expr = *value.right;
      attrs.push((name, expr));

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
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut vec = Vec::new();

    while !input.is_empty() {
      let lookahead = input.lookahead1();

      // Make sure that there's a brace for an object ahead.
      if lookahead.peek(syn::token::Brace) {
        let content;
        braced!(content in input);

        // Parse the insides of the braces with the original parser
        let attrs: Attributes = content.parse()?;
        vec.push(attrs);

        parse_until(input, Token![,])?;
      } else {
        // Else we return an error
        return Err(lookahead.error());
      }

      if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
      }
    }

    Ok(AttributeArray(vec))
  }
}

impl AttributeArray {
  fn to_tokens(&self, struct_type: TokenStream) -> syn::Result<TokenStream> {
    let structs = self.0.iter().map(|attrs| {
      quote! {
        #struct_type {
          #attrs,
          ..Default::default()
        }
      }
    });

    Ok(quote! {
      vec![
        #( #structs ),*
      ]
    })
  }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> syn::Result<TokenStream> {
  let mut tokens = TokenStream::new();
  while !input.is_empty() && !input.peek(end) {
    let next: TokenTree = input.parse()?;
    tokens.extend(Some(next));
  }
  Ok(tokens)
}

fn get_struct_type(name: &Ident) -> syn::Result<TokenStream> {
  match name.to_string().as_str() {
    "options" => Ok(quote! { slashook::structs::interactions::ApplicationCommandOption }),
    "subcommand_groups" => Ok(quote! { slashook::commands::SubcommandGroup }),
    "subcommands" => Ok(quote! { slashook::commands::Subcommand }),
    "choices" => Ok(quote! { slashook::structs::interactions::ApplicationCommandOptionChoice }),
    _ => Err(syn::Error::new(name.span(), "Unexpected field for a struct array type")),
  }
}
