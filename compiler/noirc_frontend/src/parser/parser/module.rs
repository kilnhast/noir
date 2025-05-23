use noirc_errors::Location;

use crate::{
    ast::{Ident, ItemVisibility, ModuleDeclaration},
    parser::{ItemKind, ParsedSubModule},
    token::{Attribute, Token},
};

use super::Parser;

impl Parser<'_> {
    /// ModOrContract
    ///     = ( 'mod' | 'contract' ) identifier ( '{' Module '}' | ';' )
    pub(super) fn parse_mod_or_contract(
        &mut self,
        ident: Ident,
        attributes: Vec<(Attribute, Location)>,
        is_contract: bool,
        visibility: ItemVisibility,
    ) -> ItemKind {
        let outer_attributes = self.validate_secondary_attributes(attributes);

        if self.eat_left_brace() {
            let contents = self.parse_module(
                true, // nested
            );
            self.eat_or_error(Token::RightBrace);
            ItemKind::Submodules(ParsedSubModule {
                visibility,
                name: ident,
                contents,
                outer_attributes,
                is_contract,
            })
        } else {
            let has_semicolon = self.eat_semicolons();
            if !has_semicolon {
                self.expected_token(Token::Semicolon);
            }
            ItemKind::ModuleDecl(ModuleDeclaration {
                visibility,
                ident,
                outer_attributes,
                has_semicolon,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_program_with_dummy_file,
        parser::{ItemKind, parser::tests::expect_no_errors},
    };

    #[test]
    fn parse_module_declaration() {
        // TODO: `contract foo;` is parsed correctly but we don't it's considered a module
        let src = "mod foo;";
        let (module, errors) = parse_program_with_dummy_file(src);
        expect_no_errors(&errors);
        assert_eq!(module.items.len(), 1);
        let item = &module.items[0];
        let ItemKind::ModuleDecl(module) = &item.kind else {
            panic!("Expected module declaration");
        };
        assert_eq!("foo", module.ident.to_string());
    }

    #[test]
    fn parse_submodule() {
        let src = "mod foo { mod bar; }";
        let (module, errors) = parse_program_with_dummy_file(src);
        expect_no_errors(&errors);
        assert_eq!(module.items.len(), 1);
        let item = &module.items[0];
        let ItemKind::Submodules(parsed_submodule) = &item.kind else {
            panic!("Expected submodules declaration");
        };
        assert!(!parsed_submodule.is_contract);
        assert_eq!("foo", parsed_submodule.name.to_string());
        assert_eq!(parsed_submodule.contents.items.len(), 1);
    }

    #[test]
    fn parse_contract() {
        let src = "contract foo {}";
        let (module, errors) = parse_program_with_dummy_file(src);
        expect_no_errors(&errors);
        assert_eq!(module.items.len(), 1);
        let item = &module.items[0];
        let ItemKind::Submodules(parsed_submodule) = &item.kind else {
            panic!("Expected submodules declaration");
        };
        assert!(parsed_submodule.is_contract);
        assert_eq!("foo", parsed_submodule.name.to_string());
        assert_eq!(parsed_submodule.contents.items.len(), 0);
    }
}
