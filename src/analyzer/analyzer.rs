use crate::{query::QueryManager, SymbolType};
use std::{collections::HashMap, sync::Arc};
use streaming_iterator::StreamingIterator;
use tree_sitter::{InputEdit, Node, Parser, QueryCursor, Tree};

#[derive(Debug)]
pub struct SymbolInfo {
    pub kind: SymbolType,
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

pub struct SymbolAnalyzer<'code> {
    queries: Arc<QueryManager>,
    parser: Parser,
    tree: Tree,
    code: &'code str,
    query_cache: HashMap<usize, Option<SymbolInfo>>,
}

impl<'code> SymbolAnalyzer<'code> {
    pub fn new(queries: Arc<QueryManager>, code: &'code str) -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&queries.language.clone().into())
            .map_err(|e| e.to_string())?;

        let tree = parser
            .parse(code, None)
            .ok_or_else(|| "Failed to parse code".to_string())?;

        Ok(SymbolAnalyzer {
            queries,
            parser,
            tree,
            code,
            query_cache: HashMap::new(),
        })
    }

    pub fn reset_code(&mut self, new_code: &'code str) {
        let tree = self
            .parser
            .parse(new_code, None)
            .expect("Error parsing code");

        self.tree = tree;
        self.code = new_code;
        self.query_cache.clear();
    }

    pub fn apply_edit(&mut self, edit: &InputEdit, new_code: &'code str) {
        self.tree.edit(edit);
        let new_tree = self
            .parser
            .parse(new_code, Some(&self.tree))
            .expect("Error parsing code");

        self.tree = new_tree;
        self.code = new_code;
        self.query_cache.clear();
    }

    fn root_node(&self) -> Node {
        self.tree.root_node()
    }

    fn get_node_text(&self, node: &Node) -> String {
        self.code[node.start_byte()..node.end_byte()].to_string()
    }

    fn print_node(&self, node: &Node, indent: usize) {
        let indent_str = "  ".repeat(indent);
        let node_text = self.get_node_text(&node);
        println!(
            "{}{} [{} - {}] - sexp {}: {:?}",
            indent_str,
            node.kind(),
            node.start_byte(),
            node.end_byte(),
            node.to_sexp(),
            node_text
        );

        for child in node.children(&mut node.walk()) {
            self.print_node(&child, indent + 1);
        }
    }

    pub fn print_tree(&self) {
        self.print_node(&self.root_node(), 0);
    }

    pub fn get_position(&self, pat: &str, offset: usize) -> Option<usize> {
        self.code.find(pat).map(|pos| pos + offset)
    }

    pub fn query_symbol(&self, position: usize) -> Option<SymbolInfo> {
        for prepared_query in self.queries.get_queries() {
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(
                &prepared_query.query,
                self.root_node(),
                self.code.as_bytes(),
            );

            while let Some(m) = matches.next() {
                for capture in m.captures.iter() {
                    let node = capture.node;
                    if node.start_byte() <= position && position < node.end_byte() {
                        let text = self.get_node_text(&node);
                        return Some(SymbolInfo {
                            kind: prepared_query.symbol_type(),
                            text,
                            start_byte: node.start_byte(),
                            end_byte: node.end_byte(),
                        });
                    }
                }
            }
        }
        None
    }
}
