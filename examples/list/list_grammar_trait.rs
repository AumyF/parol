// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

// Disable clippy warnings that can result in the way how parol generates code.
#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::upper_case_acronyms)]

use crate::list_grammar::ListGrammar;
use parol_runtime::parser::{ParseTreeType, UserActionsTrait};
use parol_runtime::{ParserError, Result, Token};
///
/// The `ListGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait ListGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// List: ListOpt /* Option */;
    ///
    fn list(&mut self, _list_opt: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// ListOpt /* `Option<T>::Some` */: Num ListRest ListOpt0 /* Option */;
    ///
    fn list_opt_0(
        &mut self,
        _num: &ParseTreeType,
        _list_rest: &ParseTreeType,
        _list_opt0: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ListOpt0 /* `Option<T>::Some` */: ",";
    ///
    fn list_opt0_0(&mut self, _comma: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ListOpt0 /* `Option<T>::None` */: ;
    ///
    fn list_opt0_1(&mut self) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ListOpt /* `Option<T>::None` */: ;
    ///
    fn list_opt_1(&mut self) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// ListRest: ListRestOpt /* Option */;
    ///
    fn list_rest(&mut self, _list_rest_opt: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// ListRestOpt /* `Option<T>::Some` */: "," Num ListRest;
    ///
    fn list_rest_opt_0(
        &mut self,
        _comma: &ParseTreeType,
        _num: &ParseTreeType,
        _list_rest: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// ListRestOpt /* `Option<T>::None` */: ;
    ///
    fn list_rest_opt_1(&mut self) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// Num: "0|[1-9][0-9]*";
    ///
    fn num(&mut self, _num: &ParseTreeType) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait<'_> for ListGrammar {
    ///
    /// This function is implemented automatically for the user's item ListGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeType],
    ) -> Result<()> {
        match prod_num {
            0 => self.list(&children[0]),
            1 => self.list_opt_0(&children[0], &children[1], &children[2]),
            2 => self.list_opt0_0(&children[0]),
            3 => self.list_opt0_1(),
            4 => self.list_opt_1(),
            5 => self.list_rest(&children[0]),
            6 => self.list_rest_opt_0(&children[0], &children[1], &children[2]),
            7 => self.list_rest_opt_1(),
            8 => self.num(&children[0]),
            _ => Err(ParserError::InternalError(format!(
                "Unhandled production number: {}",
                prod_num
            ))
            .into()),
        }
    }
    fn on_comment_parsed(&mut self, _token: Token<'_>) {
        // This is currently only supported for auto generate mode.
        // Please, file an issue if need arises.
    }
}
