// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use encoder::vir;
use std::fmt;
use std::iter::FlatMap;
use std::collections::HashMap;
use utils::to_string::ToString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Fold(String, Vec<vir::Expr>),
    Unfold(String, Vec<vir::Expr>)
}

impl Action {
    pub fn to_stmt(&self) -> vir::Stmt {
        match self {
            Action::Fold(ref pred, ref args) => vir::Stmt::Fold(pred.clone(), args.clone()),
            Action::Unfold(ref pred, ref args) => vir::Stmt::Unfold(pred.clone(), args.clone()),
        }
    }

    pub fn to_expr(&self, inner_expr: vir::Expr) -> vir::Expr {
        match self {
            Action::Fold(ref pred, ref args) => {
                // Currently unsupported in Viper
                unimplemented!()
            },
            Action::Unfold(ref pred, ref args) => {
                vir::Expr::Unfolding(pred.clone(), args.clone(), box inner_expr)
            }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Fold(ref pred, ref args) => write!(f, "fold {}({})", pred, args.iter().to_string()),
            Action::Unfold(ref pred, ref args) => write!(f, "unfold {}({})", pred, args.iter().to_string()),
        }
    }
}
