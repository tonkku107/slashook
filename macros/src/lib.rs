// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![warn(clippy::all)]
extern crate proc_macro;

mod converter;
mod attr_parser;

use converter::convert_function;
use attr_parser::Attributes;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use proc_macro2::Span;
use devise::{Spanned, ext::SpanDiagnosticExt};
use syn::{self, parse_macro_input, parse_quote_spanned, FnArg, ItemFn, Path, Stmt};

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
///   res.send_message("Command executed").await?;
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
///   res.send_message("Command executed").await?;
///   Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
  let attrs = parse_macro_input!(attr as Attributes);
  let function = convert_function(parse_macro_input!(item as ItemFn));
  let func_ident = function.sig.ident.clone();

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

/// A macro that turns a function to an `Event`
///
/// An `EventType` is required as an argument.
/// ## Example
/// ```ignore
/// #[event(EventType::APPLICATION_AUTHORIZED)]
/// fn authorized(event: EventInput, data: ApplicationAuthorizedEventData) {
///   event.ack().await?;
/// }
/// ```
/// ## Conversion
/// The event handler expects functions to be `async fn(EventInput, EventData) -> CmdResult`.
/// However, this macro will convert simple `fn(EventInput, <specific event data struct>) -> ()` functions into ones suitable for the event handler.\
/// This conversion provides great convenience for the simplest of events, but it is still recommended to make sure you have the correct return type from an async function so your code looks syntatically correct.
///
/// For example, the example above would be converted to:
/// ```ignore
/// async fn authorized(event: EventInput, data: EventData) -> CmdResult {
///   let data: ApplicationAuthorizedEventData = match data {...}
///   event.ack().await?;
///   Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
  let path = parse_macro_input!(attr as Path);
  let mut function = convert_function(parse_macro_input!(item as ItemFn));
  let func_ident = function.sig.ident.clone();

  let Some(FnArg::Typed(data_var)) = function.sig.inputs.get_mut(1) else {
    return syn::Error::new(function.sig.inputs.span(), "Second argument to event handler is invalid").into_compile_error().into()
  };

  let event_type = path.segments.last().unwrap().ident.to_string();
  let matcher = match event_type.as_str() {
    "APPLICATION_AUTHORIZED" => quote_spanned! {data_var.ty.span()=> slashook::structs::events::EventData::ApplicationAuthorized(d) => d},
    "ENTITLEMENT_CREATE" => quote_spanned! {data_var.ty.span()=>slashook::structs::events::EventData::EntitlementCreate(d) => d},
    "QUEST_USER_ENROLLMENT" => quote_spanned! {data_var.ty.span()=>slashook::structs::events::EventData::QuestUserEnrollment(d) => d},
    _ => return syn::Error::new(path.span(), "Unknown event type").into_compile_error().into(),
  };

  let data_var_name = data_var.pat.clone();
  let stmt: Stmt = parse_quote_spanned! {data_var.span()=> let #data_var = match #data_var_name {
    #matcher,
    _ => panic!("Unexpected event type to data type mismatch"),
  };};

  function.block.stmts.insert(0, stmt);
  data_var.ty = parse_quote_spanned! {data_var.ty.span()=> slashook::structs::events::EventData};

  let output = quote! {
    #function
    let #func_ident = slashook::events::Event {
      event_type: #path,
      func: Box::new(#func_ident),
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
