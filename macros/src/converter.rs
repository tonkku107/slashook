// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use syn::{self, Block, Stmt, Expr, parse_quote};

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
