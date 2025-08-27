// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use syn::{self, parse_quote, Block, Expr, ItemFn, ReturnType, Stmt};

pub(crate) fn convert_function(mut function: ItemFn) -> ItemFn {
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

  function
}

pub(crate) fn convert_block(block: Block) -> Block {
  let existing_statements = block.stmts;
  let mut new_statements: Vec<Stmt> = Vec::new();

  for statement in existing_statements.into_iter() {
    let expression = match statement {
      Stmt::Expr(expr, _) => expr,
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
      parse_quote! {
        {
          #inner;
          return Ok(());
        }
      }
    },
    Expr::Block(blokky) => {
      let new_block = convert_block(blokky.block);
      parse_quote!(#new_block)
    },
    Expr::If(mut iffy) => {
      iffy.then_branch = convert_block(iffy.then_branch);
      iffy.else_branch = iffy.else_branch.map(|(token, expr)| (token, Box::new(convert_expr(*expr))));
      parse_quote!(#iffy)
    },
    Expr::ForLoop(mut loopy) => {
      loopy.body = convert_block(loopy.body);
      parse_quote!(#loopy)
    },
    Expr::Loop(mut loopy) => {
      loopy.body = convert_block(loopy.body);
      parse_quote!(#loopy)
    },
    Expr::While(mut while_loopy) => {
      while_loopy.body = convert_block(while_loopy.body);
      parse_quote!(#while_loopy)
    },
    Expr::Match(mut matchy) => {
      let arms = matchy.arms;
      let mut new_arms = Vec::new();
      for mut arm in arms.into_iter() {
        arm.body = Box::new(convert_expr(*arm.body));
        new_arms.push(arm);
      }
      matchy.arms = new_arms;
      parse_quote!(#matchy)
    },
    _ => expression
  }
}
