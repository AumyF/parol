use crate::{
    assign_operator::AssignOperator, binary_operator::BinaryOperator, calc_grammar_trait::*,
    errors::CalcError,
};
use log::trace;
use miette::{bail, miette, Result};
use parol_runtime::{errors::FileSource, lexer::Token};
use std::{
    collections::BTreeMap,
    convert::TryInto,
    fmt::{Debug, Display, Error, Formatter},
    marker::PhantomData,
    str::FromStr,
};

///
/// The value range for the supported calculations
///
pub type DefinitionRange = isize;

///
/// Data structure that implements the semantic actions for our calc grammar
///
#[derive(Debug, Default)]
pub struct CalcGrammar<'t> {
    pub calc_results: Vec<DefinitionRange>,
    pub env: BTreeMap<String, DefinitionRange>,
    phantom: PhantomData<&'t str>, // Just to hold the lifetime generated by parol
}

impl<'t> CalcGrammar<'t> {
    pub fn new() -> Self {
        CalcGrammar::default()
    }

    fn value(&self, id: &Token<'t>) -> Result<DefinitionRange> {
        self.env
            .get(id.text())
            .cloned()
            .ok_or(miette!(CalcError::UndeclaredVariable {
                context: "value".to_owned(),
                input: FileSource::try_new(id.location.file_name.clone())?.into(),
                token: id.into()
            }))
    }

    fn declare(&mut self, id: &str, context: &str) {
        if !self.env.contains_key(id) {
            trace!("declare {}: {}", context, id);
            self.env.insert(id.to_owned(), 0);
        }
    }

    fn apply_assign_operation(
        lhs: &mut DefinitionRange,
        op: &AssignOperator,
        rhs: DefinitionRange,
        context: &str,
    ) -> Result<DefinitionRange> {
        trace!("apply_assign_item: {}: {} {} {}", context, lhs, op, rhs);
        match op {
            AssignOperator::Assign => *lhs = rhs,
            AssignOperator::Plus => *lhs += rhs,
            AssignOperator::Minus => *lhs -= rhs,
            AssignOperator::Mul => *lhs *= rhs,
            AssignOperator::Div => {
                if rhs == 0 {
                    bail!("Division by zero detected!");
                }
                *lhs /= rhs
            }
            AssignOperator::Mod => *lhs %= rhs,
            AssignOperator::ShiftLeft => *lhs <<= rhs,
            AssignOperator::ShiftRight => *lhs >>= rhs,
            AssignOperator::BitwiseAnd => *lhs &= rhs,
            AssignOperator::BitwiseXOr => *lhs ^= rhs,
            AssignOperator::BitwiseOr => *lhs |= rhs,
        }
        trace!("apply_assign_item:      = {}", lhs);
        Ok(*lhs)
    }

    fn apply_binary_operation(
        lhs: DefinitionRange,
        op: &BinaryOperator,
        rhs: DefinitionRange,
        context: &str,
    ) -> Result<DefinitionRange> {
        trace!(
            "apply_binary_operation: {}: {} {} {}",
            context,
            lhs,
            op,
            rhs
        );
        let result = match op {
            BinaryOperator::Add => lhs + rhs,
            BinaryOperator::Sub => lhs - rhs,
            BinaryOperator::Mul => lhs * rhs,
            BinaryOperator::Div => {
                if rhs == 0 {
                    bail!("Division by zero detected!");
                }
                lhs / rhs
            }
            BinaryOperator::Mod => lhs % rhs,
            BinaryOperator::Pow => {
                if let Ok(exponent) = rhs.try_into() {
                    lhs.pow(exponent)
                } else {
                    bail!("Exponent {} can't be converted to u32!", rhs);
                }
            }
            BinaryOperator::Eq => (lhs == rhs) as DefinitionRange,
            BinaryOperator::Ne => (lhs != rhs) as DefinitionRange,
            BinaryOperator::Lt => (lhs < rhs) as DefinitionRange,
            BinaryOperator::Le => (lhs <= rhs) as DefinitionRange,
            BinaryOperator::Gt => (lhs > rhs) as DefinitionRange,
            BinaryOperator::Ge => (lhs >= rhs) as DefinitionRange,
            BinaryOperator::BitShl => lhs << rhs,
            BinaryOperator::BitShr => lhs >> rhs,
            BinaryOperator::BitAnd => lhs & rhs,
            BinaryOperator::BitOr => lhs | rhs,
            BinaryOperator::LogAnd => ((lhs != 0) && (rhs != 0)) as DefinitionRange,
            BinaryOperator::LogOr => ((lhs != 0) || (rhs != 0)) as DefinitionRange,
        };

        trace!("apply_binary_operation:      = {}", result);

        Ok(result)
    }

    fn process_calc(&mut self, calc: &Calc) -> Result<()> {
        calc.calc_list.iter().fold(Ok(()), |res, elem| {
            res?;
            self.process_calc_list(elem)
        })
    }

    fn process_calc_list(&mut self, elem: &CalcList) -> Result<()> {
        self.process_instruction(&elem.instruction)
    }

    fn process_instruction(&mut self, insn: &Instruction) -> Result<()> {
        match insn {
            Instruction::Assignment(ins) => self.process_assignment(&ins.assignment),
            Instruction::LogicalOr(ins) => self
                .process_logical_or(&ins.logical_or)
                .map(|r| self.calc_results.push(r)),
        }
    }

    fn process_assignment(&mut self, assignment: &Assignment) -> Result<()> {
        let context = "process_assignment";
        let mut result = self.process_logical_or(&assignment.logical_or)?;
        let mut assignment_list = assignment.assignment_list.clone();
        // Prepend the left most (mandatory) assign item
        assignment_list.insert(
            0,
            AssignmentList {
                assign_item: assignment.assign_item.clone(),
            },
        );
        // Assign from right to left (right associative)
        for assign_item in assignment_list.iter().rev() {
            let id = assign_item.assign_item.id.id.text();
            let op = assign_item
                .assign_item
                .assign_op
                .assign_op
                .text()
                .try_into()?;
            self.declare(id, context);
            if let Some(var) = self.env.get_mut(id) {
                trace!("assign: to variable {}", id);
                result = Self::apply_assign_operation(var, &op, result, context)?;
            } else {
                return Err(miette!(CalcError::UndeclaredVariable {
                    context: "value".to_owned(),
                    input: FileSource::try_new(
                        assign_item.assign_item.id.id.location.file_name.clone()
                    )?
                    .into(),
                    token: (&assign_item.assign_item.id.id).into()
                }));
            }
        }
        Ok(())
    }

    fn process_logical_or(&mut self, logical_or: &LogicalOr) -> Result<DefinitionRange> {
        let context = "process_logical_or";
        let mut result = self.process_logical_and(&logical_or.logical_and)?;
        for item in &logical_or.logical_or_list {
            let op: BinaryOperator = item.logical_or_op.logical_or_op.text().try_into()?;
            let next_operand = self.process_logical_and(&item.logical_and)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_logical_and(&mut self, logical_and: &LogicalAnd) -> Result<DefinitionRange> {
        let context = "process_logical_and";
        let mut result = self.process_bitwise_or(&logical_and.bitwise_or)?;
        for item in &logical_and.logical_and_list {
            let op: BinaryOperator = item.logical_and_op.logical_and_op.text().try_into()?;
            let next_operand = self.process_bitwise_or(&item.bitwise_or)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_bitwise_or(&mut self, bitwise_or: &BitwiseOr) -> Result<DefinitionRange> {
        let context = "process_bitwise_or";
        let mut result = self.process_bitwise_and(&bitwise_or.bitwise_and)?;
        for item in &bitwise_or.bitwise_or_list {
            let op: BinaryOperator = item.bitwise_or_op.bitwise_or_op.text().try_into()?;
            let next_operand = self.process_bitwise_and(&item.bitwise_and)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_bitwise_and(&mut self, bitwise_and: &BitwiseAnd) -> Result<DefinitionRange> {
        let context = "process_bitwise_and";
        let mut result = self.process_equality(&bitwise_and.equality)?;
        for item in &bitwise_and.bitwise_and_list {
            let op: BinaryOperator = item.bitwise_and_op.bitwise_and_op.text().try_into()?;
            let next_operand = self.process_equality(&item.equality)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_equality(&mut self, equality: &Equality) -> Result<DefinitionRange> {
        let context = "process_equality";
        let mut result = self.process_relational(&equality.relational)?;
        for item in &equality.equality_list {
            let op: BinaryOperator = item.equality_op.equality_op.text().try_into()?;
            let next_operand = self.process_relational(&item.relational)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_relational(&mut self, relational: &Relational) -> Result<DefinitionRange> {
        let context = "process_relational";
        let mut result = self.process_bitwise_shift(&relational.bitwise_shift)?;
        for item in &relational.relational_list {
            let op: BinaryOperator = item.relational_op.relational_op.text().try_into()?;
            let next_operand = self.process_bitwise_shift(&item.bitwise_shift)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_bitwise_shift(&mut self, bitwise_shift: &BitwiseShift) -> Result<DefinitionRange> {
        let context = "process_bitwise_shift";
        let mut result = self.process_sum(&bitwise_shift.summ)?;
        for item in &bitwise_shift.bitwise_shift_list {
            let op: BinaryOperator = item.bitwise_shift_op.bitwise_shift_op.text().try_into()?;
            let next_operand = self.process_sum(&item.summ)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_sum(&mut self, summ: &Summ) -> Result<DefinitionRange> {
        let context = "process_sum";
        let mut result = self.process_mult(&summ.mult)?;
        for item in &summ.summ_list {
            let op: BinaryOperator = match &*item.add_op {
                AddOp::Plus(plus) => plus.plus.plus.text().try_into(),
                AddOp::Minus(minus) => minus.minus.minus.text().try_into(),
            }?;
            let next_operand = self.process_mult(&item.mult)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_mult(&mut self, mult: &Mult) -> Result<DefinitionRange> {
        let context = "process_mult";
        let mut result = self.process_power(&mult.power)?;
        for item in &mult.mult_list {
            let op: BinaryOperator = item.mult_op.mult_op.text().try_into()?;
            let next_operand = self.process_power(&item.power)?;
            result = Self::apply_binary_operation(result, &op, next_operand, context)?;
        }
        Ok(result)
    }

    fn process_power(&mut self, power: &Power) -> Result<DefinitionRange> {
        let context = "process_power";
        let op = BinaryOperator::Pow;
        // Calculate from right to left (power operation is right associative)
        let result = power
            .power_list
            .iter()
            .rev()
            .fold(Ok(1), |acc, f| match acc {
                Ok(_) => {
                    let val = self.process_factor(&f.factor)?;
                    Self::apply_binary_operation(val, &op, acc.unwrap(), context)
                }
                Err(_) => acc,
            })?;
        Self::apply_binary_operation(self.process_factor(&power.factor)?, &op, result, context)
    }

    fn process_factor(&mut self, factor: &Factor) -> Result<DefinitionRange> {
        match factor {
            Factor::Number(FactorNumber { number }) => Ok(number.number.0),
            Factor::IdRef(FactorIdRef { id_ref }) => self.value(&id_ref.id.id),
            Factor::NegateFactor(FactorNegateFactor { factor, .. }) => {
                Ok(-(self.process_factor(factor)?))
            }
            Factor::LParenLogicalOrRParen(FactorLParenLogicalOrRParen { logical_or, .. }) => {
                self.process_logical_or(logical_or)
            }
        }
    }
}

impl Display for CalcGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(
            f,
            "Unassigned results\n{}",
            self.calc_results
                .iter()
                .rev()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join("\n")
        )?;
        writeln!(
            f,
            "\nEnv\n{}",
            self.env
                .iter()
                .map(|(i, v)| format!("{} = {}", i, v))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl<'t> CalcGrammarTrait<'t> for CalcGrammar<'t> {
    /// Semantic action for non-terminal 'Calc'
    fn calc(&mut self, arg: &Calc<'t>) -> Result<()> {
        self.process_calc(arg)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Number(isize);

impl<'t> TryFrom<&Token<'t>> for Number {
    type Error = <isize as FromStr>::Err;

    fn try_from(number: &Token<'t>) -> Result<Self, Self::Error> {
        Ok(Self(number.text().parse::<isize>()?))
    }
}
