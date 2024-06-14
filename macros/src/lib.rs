// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![warn(clippy::all)]
extern crate proc_macro;

mod converter;
mod attr_parser;

use converter::convert_block;
use attr_parser::Attributes;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use proc_macro2::Span;
use devise::{Spanned, ext::SpanDiagnosticExt};
use syn::{self, ItemFn, ReturnType, parse_macro_input, parse_quote};

/// A macro that turns a function to a `Command`
///
/// At minimum, a `name` field is required for basic operation.\
/// You can also add additional fields for `Command` following the application command structure to sync your commands with Discord using `Client::sync_commands`.\
/// These fields are formatted `name = value` and are comma separated.\
/// `into` is called for every value and missing fields are filled with defaults to make things easier.\
/// Instead of creating subcommands as options, you can use `subcommand_groups` and `subcommands`.\
/// `Vec`s of values can be constructed by simply using `[]` and comma separating the values, structs and maps can be done with `{}` following the same syntax inside.\
/// If you're creating a "fake" command (as a separate component handler for example), you can set `ignore = true` to make sure that command isn't synced.
/// ## Example
/// ```ignore
/// #[command(
///   name = "command-name",
///   description = "A cool command",
///   integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
///   contexts = [InteractionContextType::GUILD, InteractionContextType::PRIVATE_CHANNEL, InteractionContextType::BOT_DM],
///   subcommand_groups = [{
///     name = "group-name",
///     subcommands = [{
///       name = "subcommand-name",
///       description = "A cool subcommand"
///       options = [{
///         name = "option",
///         description = "A cool description",
///         option_type = InteractionOptionType::STRING,
///         required = true
///       }]
///     }]
///   }]
/// )]
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
  let attrs = parse_macro_input!(attr as Attributes);
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
      #attrs,
      ..Default::default()
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
