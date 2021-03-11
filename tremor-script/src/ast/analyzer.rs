// Copyright 2020-2021, The Tremor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
    ArrayPattern, ArrayPredicatePattern, AssignPattern, ImutExprInt, Pattern, PredicateClause,
    PredicatePattern, RecordPattern, TestExpr, TuplePattern,
};

pub(crate) trait Costly {
    fn cost(&self) -> u64;
}

impl Costly for TestExpr {
    fn cost(&self) -> u64 {
        self.extractor.cost()
    }
}

impl<'script> Costly for PredicateClause<'script> {
    fn cost(&self) -> u64 {
        let g = if self.guard.is_some() { 10 } else { 0 };
        g + self.pattern.cost()
    }
}

impl<'script> Costly for Pattern<'script> {
    fn cost(&self) -> u64 {
        match self {
            Pattern::DoNotCare | Pattern::Default => 0,
            Pattern::Expr(_) => 10,
            Pattern::Record(r) => r.cost(),
            Pattern::Array(a) => a.cost(),
            Pattern::Assign(a) => a.cost(),
            Pattern::Tuple(t) => t.cost(),
            Pattern::Extract(e) => e.cost(),
        }
    }
}

impl<'script> Costly for PredicatePattern<'script> {
    fn cost(&self) -> u64 {
        match self {
            PredicatePattern::FieldPresent { .. } | PredicatePattern::FieldAbsent { .. } => 10,
            PredicatePattern::Bin {
                rhs: ImutExprInt::Literal(_),
                ..
            } => 20,
            PredicatePattern::Bin { .. } => 100,
            PredicatePattern::TildeEq { test, .. } => test.cost(),
            PredicatePattern::RecordPatternEq { pattern, .. } => pattern.cost(),
            PredicatePattern::ArrayPatternEq { pattern, .. } => pattern.cost(),
        }
    }
}

impl<'script> Costly for RecordPattern<'script> {
    fn cost(&self) -> u64 {
        self.fields.iter().map(|f| f.cost() + 100).sum()
    }
}

impl<'script> Costly for ArrayPredicatePattern<'script> {
    fn cost(&self) -> u64 {
        match self {
            ArrayPredicatePattern::Ignore => 1,
            ArrayPredicatePattern::Expr(ImutExprInt::Literal(_))
            | ArrayPredicatePattern::Expr(_) => 10,
            ArrayPredicatePattern::Tilde(t) => t.cost(),
            ArrayPredicatePattern::Record(r) => r.cost(),
        }
    }
}

impl<'script> Costly for ArrayPattern<'script> {
    fn cost(&self) -> u64 {
        let s: u64 = self.exprs.iter().map(Costly::cost).sum();
        s * (self.exprs.len() as u64)
    }
}

impl<'script> Costly for AssignPattern<'script> {
    fn cost(&self) -> u64 {
        self.pattern.cost()
    }
}
impl<'script> Costly for TuplePattern<'script> {
    fn cost(&self) -> u64 {
        self.exprs.iter().map(Costly::cost).sum()
    }
}

/*
  exclusivity
*/

// pub(crate) trait Exclusive {
//     fn is_exclusive_to(&self, other: &Self) -> bool;
// }
