// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![warn(clippy::all)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use proc_macro2::{Span};
use devise::{Spanned, ext::SpanDiagnosticExt};
use syn::{self, LitStr, ItemFn, ReturnType, Block, Stmt, Expr, parse_macro_input, parse_quote};

/// A macro that turns a function to a Command
///
/// A command name is required as an argument.
/// ## Example
/// ```ignore
/// #[command("command name")]
/// fn command(input: CommandInput, res: CommandResponder) {
///   res.send_message("Command executed")?;
/// }
/// ```
/// ## Conversion
/// The command handler expects functions to be `async fn(CommandInput, CommandResponder) -> CmdResult`.
/// However, this macro will convert simple `fn(CommandInput, CommandResponder) -> ()` functions into ones suitable for the command handler.\
/// This conversion provides great convenience for the simplest of commands, but it is still recommended to make sure you have the correct return type from an async function so your code looks syntatically correct.
///
/// For example, the example above would be converted to:
/// ```ignore
/// async fn command(input: CommandInput, res: CommandResponder) -> CmdResult {
///   res.send_message("Command executed")?;
///   Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
  let command_name = parse_macro_input!(attr as LitStr).value();
  let mut function = parse_macro_input!(item as ItemFn);
  let func_ident = function.sig.ident.clone();

  // Force function to be async
  if function.sig.asyncness.is_none() {
    function.sig.asyncness = parse_quote!(async);
  }

  // Convert functions that return () to ones that return a Result
  if let ReturnType::Default = function.sig.output {
    function.sig.output = parse_quote!(-> slashook::commands::CmdResult);
    let converted_block = convert_block(*function.block);
    let statements = converted_block.stmts;
    let new_block = parse_quote!{
      {
        #(#statements)*;
        #[allow(unreachable_code)]
        Ok(())
      }
    };
    function.block = Box::new(new_block);
  }

  let output = quote! {
    #function
    let #func_ident = slashook::commands::Command {
      func: Box::new(#func_ident),
      name: #command_name.to_string()
    };
  };
  output.into()
}

// Reimplementation of Rocket's main macro so that we can use the re-exported rocket without having to add rocket as a dependency
/// Sets up an async runtime
///
/// You may also use tokio directly instead of this macro.
/// See also: [Rocket's documentation](https://api.rocket.rs/v0.5-rc/rocket/attr.main.html) and [Tokio's documentation](https://docs.rs/tokio/1.11.0/tokio/attr.main.html)
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
  let function = parse_macro_input!(item as ItemFn);
  let mut sig = function.sig;

  if sig.ident != "main" {
    Span::call_site()
      .warning("attribute is typically applied to `main` function")
      .span_note(sig.ident.span(), "this function is not `main`")
      .emit_as_item_tokens();
  }

  sig.asyncness = None;
  let block = function.block;
  let attrs = function.attrs;
  let vis = function.vis;
  quote_spanned!(block.span() => #(#attrs)* #vis #sig {
    slashook::async_main(async move #block)
  }).into()
}

fn convert_block(block: Block) -> Block {
  let existing_statements = block.stmts;
  let mut new_statements: Vec<Stmt> = Vec::new();

  for statement in existing_statements.into_iter() {
    let expression = match statement {
      Stmt::Expr(expr) => expr,
      Stmt::Semi(expr, _) => expr,
      _ => {
        new_statements.push(statement);
        continue
      }
    };

    let new_expr = convert_expr(expression);
    new_statements.push(parse_quote!(#new_expr;));
  }

  parse_quote! {
    {
      #(#new_statements)*
    }
  }
}

fn convert_expr(expression: Expr) -> Expr {
  match expression {
    Expr::Return(ret) => {
      let inner = ret.expr;
      return parse_quote! {
        {
          #inner;
          return Ok(());
        }
      }
    },
    Expr::Block(blokky) => {
      let new_block = convert_block(blokky.block);
      return parse_quote!(#new_block);
    },
    Expr::If(mut iffy) => {
      iffy.then_branch = convert_block(iffy.then_branch);
      iffy.else_branch = iffy.else_branch.map(|(token, expr)| (token, Box::new(convert_expr(*expr))));
      return parse_quote!(#iffy);
    },
    Expr::ForLoop(mut loopy) => {
      loopy.body = convert_block(loopy.body);
      return parse_quote!(#loopy);
    },
    Expr::Loop(mut loopy) => {
      loopy.body = convert_block(loopy.body);
      return parse_quote!(#loopy);
    },
    Expr::While(mut while_loopy) => {
      while_loopy.body = convert_block(while_loopy.body);
      return parse_quote!(#while_loopy);
    },
    Expr::Match(mut matchy) => {
      let arms = matchy.arms;
      let mut new_arms = Vec::new();
      for mut arm in arms.into_iter() {
        arm.body = Box::new(convert_expr(*arm.body));
        new_arms.push(arm);
      }
      matchy.arms = new_arms;
      return parse_quote!(#matchy);
    },
    _ => expression
  }
}
