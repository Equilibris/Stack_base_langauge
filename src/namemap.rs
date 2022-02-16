use std::collections::HashMap;

use anyhow::bail;

use crate::{
    ast::{ASTNode, TopLevelNode},
    numeric_litteral::NumericLiteral,
};

#[derive(Debug)]
pub enum NameMapNode {
    Word {
        implementation: Vec<ASTNode>,
        depends_on: Vec<String>,
    },
    AliasedWord(String),

    StringConst(String),
    NumericConst(NumericLiteral),
}
pub type NameMap = HashMap<String, NameMapNode>;

pub fn extract_name_map(base: Vec<TopLevelNode>) -> anyhow::Result<NameMap> {
    let mut map = NameMap::new();

    for node in base {
        match node {
            TopLevelNode::WordDeclare(ident, implementation) => match map.insert(
                ident,
                match implementation {
                    ASTNode::Curly(a) => {
                        let mut depends_on = Vec::new();

                        for value in a.iter() {
                            if let ASTNode::Ident(s) = value {
                                depends_on.push(s.clone())
                            }
                        }
                        NameMapNode::Word {
                            implementation: a,
                            depends_on,
                        }
                    }
                    ASTNode::Ident(a) => NameMapNode::AliasedWord(a),
                    ASTNode::NumericLiteral(a) => NameMapNode::NumericConst(a),
                    ASTNode::StringLiteral(s) => NameMapNode::StringConst(s),
                    _ => bail!("Bad word declaration value"),
                },
            ) {
                Some(x) => todo!(),
                None => (),
            },
            TopLevelNode::Typing(_, _) => todo!(),
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::namemap::extract_name_map;

    #[test]
    fn exp() {
        let program = "@main { \"Hello world!\" . } ; this be hello world".to_string();
        let program = crate::tokenizer::tokenizer(program).unwrap();
        let program = crate::ast::build_tree(program).unwrap();
        let program = extract_name_map(program).unwrap();

        println!("{:?}", program);
    }
}
