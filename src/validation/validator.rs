use crate::validation::node::RustNode;
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use syn::{
    parse_file,
    spanned::Spanned,
    visit::{self, Visit},
    UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree,
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line:{},col:{}]", self.line, self.column)
    }
}

pub enum ValidationError {
    Parse(syn::Error),
    Invalid(InvalidError),
}

pub struct InvalidError {
    pub invalid_node_locations: BTreeMap<RustNode, BTreeSet<Location>>,
    pub invalid_path_locations: BTreeMap<String, BTreeSet<Location>>,
}

pub struct Validator {
    whitelist_nodes: HashSet<RustNode>,
    whitelist_path_prefix: HashSet<String>,
    whitelist_path_prefix_crate: bool,
    whitelist_path_prefix_super: bool,
    whitelist_path_prefix_self: bool,
    whitelist_path_prefix_simple: bool,
    invalid_node_locations: BTreeMap<RustNode, BTreeSet<Location>>,
    invalid_path_locations: BTreeMap<String, BTreeSet<Location>>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            whitelist_nodes: HashSet::new(),
            whitelist_path_prefix: HashSet::new(),
            whitelist_path_prefix_crate: false,
            whitelist_path_prefix_self: false,
            whitelist_path_prefix_super: false,
            whitelist_path_prefix_simple: false,
            invalid_node_locations: BTreeMap::new(),
            invalid_path_locations: BTreeMap::new(),
        }
    }
    pub fn allow_node(&mut self, syntax: RustNode) -> &mut Self {
        self.whitelist_nodes.insert(syntax);
        self
    }
    pub fn allow_path_prefix(&mut self, path_prefix: impl Into<String>) -> &mut Self {
        self.whitelist_path_prefix.insert(path_prefix.into());
        self
    }
    pub fn allow_relative_path_crate(&mut self) -> &mut Self {
        self.whitelist_path_prefix_crate = true;
        self
    }
    pub fn allow_relative_path_super(&mut self) -> &mut Self {
        self.whitelist_path_prefix_super = true;
        self
    }
    pub fn allow_relative_path_self(&mut self) -> &mut Self {
        self.whitelist_path_prefix_self = true;
        self
    }
    pub fn allow_relative_path_simple(&mut self) -> &mut Self {
        self.whitelist_path_prefix_simple = true;
        self
    }
    pub fn validate(&mut self, code: &str) -> Result<(), ValidationError> {
        match parse_file(code) {
            Ok(ast) => {
                self.invalid_node_locations.clear();
                self.visit_file(&ast);
                let invalid_node_locations = std::mem::take(&mut self.invalid_node_locations);
                let invalid_path_locations = std::mem::take(&mut self.invalid_path_locations);
                if invalid_node_locations.is_empty() && invalid_path_locations.is_empty() {
                    return Ok(());
                }
                Err(ValidationError::Invalid(InvalidError {
                    invalid_node_locations,
                    invalid_path_locations,
                }))
            }
            Err(e) => Err(ValidationError::Parse(e)),
        }
    }

    fn check_node(&mut self, node: RustNode, span: impl Spanned) {
        if !self.whitelist_nodes.contains(&node) {
            let start = span.span().start();
            let location = Location {
                line: start.line,
                column: start.column,
            };
            self.invalid_node_locations
                .entry(node)
                .or_default()
                .insert(location);
        }
    }
    fn check_path(&mut self, path: &str, span: impl Spanned) {
        let mut is_whitelisted = false;
        if self.whitelist_path_prefix_crate && path.starts_with("crate::") {
            is_whitelisted = true;
        }
        if self.whitelist_path_prefix_self && path.starts_with("self::") {
            is_whitelisted = true;
        }
        if self.whitelist_path_prefix_super && path.starts_with("super::") {
            is_whitelisted = true;
        }
        if self.whitelist_path_prefix_simple && !path.contains("::") {
            is_whitelisted = true;
        }
        self.whitelist_path_prefix.iter().for_each(|prefix| {
            if path.starts_with(prefix) {
                is_whitelisted = true;
            }
        });
        if !is_whitelisted {
            let start = span.span().start();
            let location = Location {
                line: start.line,
                column: start.column,
            };
            self.invalid_path_locations
                .entry(path.to_string())
                .or_default()
                .insert(location);
        }
    }
    fn collate_paths(&mut self, tree: &UseTree) -> BTreeSet<String> {
        let mut paths = BTreeSet::new();
        self.collate_paths_with_prefix(tree, &mut paths, String::new());
        paths
    }
    fn collate_paths_with_prefix(
        &mut self,
        tree: &UseTree,
        paths: &mut BTreeSet<String>,
        prefix: String,
    ) {
        match tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                let new_prefix = if prefix.is_empty() {
                    ident.to_string()
                } else {
                    format!("{}::{}", prefix, ident)
                };
                self.collate_paths_with_prefix(tree, paths, new_prefix);
            }
            UseTree::Group(UseGroup { items, .. }) => {
                for item in items {
                    self.collate_paths_with_prefix(item, paths, prefix.clone());
                }
            }
            UseTree::Name(UseName { ident }) => {
                let full = if prefix.is_empty() {
                    ident.to_string()
                } else {
                    format!("{}::{}", prefix, ident)
                };
                paths.insert(full);
            }
            UseTree::Rename(UseRename { ident, .. }) => {
                let full = if prefix.is_empty() {
                    ident.to_string()
                } else {
                    format!("{}::{}", prefix, ident)
                };
                paths.insert(full);
            }
            UseTree::Glob(UseGlob { .. }) => {
                let full = if prefix.is_empty() {
                    "*".to_string()
                } else {
                    format!("{}::*", prefix)
                };
                paths.insert(full);
            }
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl syn::visit::Visit<'_> for Validator {
    fn visit_abi(&mut self, node: &syn::Abi) {
        self.check_node(RustNode::Abi, node.span());
        visit::visit_abi(self, node);
    }

    fn visit_angle_bracketed_generic_arguments(
        &mut self,
        node: &syn::AngleBracketedGenericArguments,
    ) {
        self.check_node(RustNode::AngleBracketedGenericArguments, node.span());
        visit::visit_angle_bracketed_generic_arguments(self, node);
    }

    fn visit_arm(&mut self, node: &syn::Arm) {
        self.check_node(RustNode::Arm, node.span());
        visit::visit_arm(self, node);
    }

    fn visit_assoc_const(&mut self, node: &syn::AssocConst) {
        self.check_node(RustNode::AssocConst, node.span());
        visit::visit_assoc_const(self, node);
    }

    fn visit_assoc_type(&mut self, node: &syn::AssocType) {
        self.check_node(RustNode::AssocType, node.span());
        visit::visit_assoc_type(self, node);
    }

    fn visit_attribute(&mut self, node: &syn::Attribute) {
        self.check_node(RustNode::Attribute, node.span());
        visit::visit_attribute(self, node);
    }

    fn visit_bare_fn_arg(&mut self, node: &syn::BareFnArg) {
        self.check_node(RustNode::BareFnArg, node.span());
        visit::visit_bare_fn_arg(self, node);
    }

    fn visit_bare_variadic(&mut self, node: &syn::BareVariadic) {
        self.check_node(RustNode::BareVariadic, node.span());
        visit::visit_bare_variadic(self, node);
    }

    fn visit_bin_op(&mut self, node: &syn::BinOp) {
        self.check_node(RustNode::BinOp, node.span());
        visit::visit_bin_op(self, node);
    }

    fn visit_block(&mut self, node: &syn::Block) {
        self.check_node(RustNode::Block, node.span());
        visit::visit_block(self, node);
    }

    fn visit_bound_lifetimes(&mut self, node: &syn::BoundLifetimes) {
        self.check_node(RustNode::BoundLifetimes, node.span());
        visit::visit_bound_lifetimes(self, node);
    }

    fn visit_captured_param(&mut self, node: &syn::CapturedParam) {
        self.check_node(RustNode::CapturedParam, node.span());
        visit::visit_captured_param(self, node);
    }

    fn visit_const_param(&mut self, node: &syn::ConstParam) {
        self.check_node(RustNode::ConstParam, node.span());
        visit::visit_const_param(self, node);
    }

    fn visit_constraint(&mut self, node: &syn::Constraint) {
        self.check_node(RustNode::Constraint, node.span());
        visit::visit_constraint(self, node);
    }

    fn visit_data_enum(&mut self, node: &syn::DataEnum) {
        self.check_node(RustNode::DataEnum, node.enum_token.span());
        visit::visit_data_enum(self, node);
    }

    fn visit_data_struct(&mut self, node: &syn::DataStruct) {
        self.check_node(RustNode::DataStruct, node.struct_token.span());
        visit::visit_data_struct(self, node);
    }

    fn visit_data_union(&mut self, node: &syn::DataUnion) {
        self.check_node(RustNode::DataUnion, node.union_token.span());
        visit::visit_data_union(self, node);
    }

    fn visit_derive_input(&mut self, node: &syn::DeriveInput) {
        self.check_node(RustNode::DeriveInput, node.span());
        visit::visit_derive_input(self, node);
    }

    fn visit_expr(&mut self, node: &syn::Expr) {
        self.check_node(RustNode::Expr, node.span());
        visit::visit_expr(self, node);
    }

    fn visit_expr_array(&mut self, node: &syn::ExprArray) {
        self.check_node(RustNode::ExprArray, node.span());
        visit::visit_expr_array(self, node);
    }

    fn visit_expr_assign(&mut self, node: &syn::ExprAssign) {
        self.check_node(RustNode::ExprAssign, node.span());
        visit::visit_expr_assign(self, node);
    }

    fn visit_expr_async(&mut self, node: &syn::ExprAsync) {
        self.check_node(RustNode::ExprAsync, node.span());
        visit::visit_expr_async(self, node);
    }

    fn visit_expr_await(&mut self, node: &syn::ExprAwait) {
        self.check_node(RustNode::ExprAwait, node.span());
        visit::visit_expr_await(self, node);
    }

    fn visit_expr_binary(&mut self, node: &syn::ExprBinary) {
        self.check_node(RustNode::ExprBinary, node.span());
        visit::visit_expr_binary(self, node);
    }

    fn visit_expr_block(&mut self, node: &syn::ExprBlock) {
        self.check_node(RustNode::ExprBlock, node.span());
        visit::visit_expr_block(self, node);
    }

    fn visit_expr_break(&mut self, node: &syn::ExprBreak) {
        self.check_node(RustNode::ExprBreak, node.span());
        visit::visit_expr_break(self, node);
    }

    fn visit_expr_call(&mut self, node: &syn::ExprCall) {
        self.check_node(RustNode::ExprCall, node.span());
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_cast(&mut self, node: &syn::ExprCast) {
        self.check_node(RustNode::ExprCast, node.span());
        visit::visit_expr_cast(self, node);
    }

    fn visit_expr_closure(&mut self, node: &syn::ExprClosure) {
        self.check_node(RustNode::ExprClosure, node.span());
        visit::visit_expr_closure(self, node);
    }

    fn visit_expr_const(&mut self, node: &syn::ExprConst) {
        self.check_node(RustNode::ExprConst, node.span());
        visit::visit_expr_const(self, node);
    }

    fn visit_expr_continue(&mut self, node: &syn::ExprContinue) {
        self.check_node(RustNode::ExprContinue, node.span());
        visit::visit_expr_continue(self, node);
    }

    fn visit_expr_field(&mut self, node: &syn::ExprField) {
        self.check_node(RustNode::ExprField, node.span());
        visit::visit_expr_field(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &syn::ExprForLoop) {
        self.check_node(RustNode::ExprForLoop, node.span());
        visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_group(&mut self, node: &syn::ExprGroup) {
        self.check_node(RustNode::ExprGroup, node.span());
        visit::visit_expr_group(self, node);
    }

    fn visit_expr_if(&mut self, node: &syn::ExprIf) {
        self.check_node(RustNode::ExprIf, node.span());
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_index(&mut self, node: &syn::ExprIndex) {
        self.check_node(RustNode::ExprIndex, node.span());
        visit::visit_expr_index(self, node);
    }

    fn visit_expr_infer(&mut self, node: &syn::ExprInfer) {
        self.check_node(RustNode::ExprInfer, node.span());
        visit::visit_expr_infer(self, node);
    }

    fn visit_expr_let(&mut self, node: &syn::ExprLet) {
        self.check_node(RustNode::ExprLet, node.span());
        visit::visit_expr_let(self, node);
    }

    fn visit_expr_lit(&mut self, node: &syn::ExprLit) {
        self.check_node(RustNode::ExprLit, node.span());
        visit::visit_expr_lit(self, node);
    }

    fn visit_expr_loop(&mut self, node: &syn::ExprLoop) {
        self.check_node(RustNode::ExprLoop, node.span());
        visit::visit_expr_loop(self, node);
    }

    fn visit_expr_macro(&mut self, node: &syn::ExprMacro) {
        self.check_node(RustNode::ExprMacro, node.span());
        visit::visit_expr_macro(self, node);
    }

    fn visit_expr_match(&mut self, node: &syn::ExprMatch) {
        self.check_node(RustNode::ExprMatch, node.span());
        visit::visit_expr_match(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &syn::ExprMethodCall) {
        self.check_node(RustNode::ExprMethodCall, node.span());
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_paren(&mut self, node: &syn::ExprParen) {
        self.check_node(RustNode::ExprParen, node.span());
        visit::visit_expr_paren(self, node);
    }

    fn visit_expr_path(&mut self, node: &syn::ExprPath) {
        self.check_node(RustNode::ExprPath, node.span());
        visit::visit_expr_path(self, node);
    }

    fn visit_expr_range(&mut self, node: &syn::ExprRange) {
        self.check_node(RustNode::ExprRange, node.span());
        visit::visit_expr_range(self, node);
    }

    fn visit_expr_raw_addr(&mut self, node: &syn::ExprRawAddr) {
        self.check_node(RustNode::ExprRawAddr, node.span());
        visit::visit_expr_raw_addr(self, node);
    }

    fn visit_expr_reference(&mut self, node: &syn::ExprReference) {
        self.check_node(RustNode::ExprReference, node.span());
        visit::visit_expr_reference(self, node);
    }

    fn visit_expr_repeat(&mut self, node: &syn::ExprRepeat) {
        self.check_node(RustNode::ExprRepeat, node.span());
        visit::visit_expr_repeat(self, node);
    }

    fn visit_expr_return(&mut self, node: &syn::ExprReturn) {
        self.check_node(RustNode::ExprReturn, node.span());
        visit::visit_expr_return(self, node);
    }

    fn visit_expr_struct(&mut self, node: &syn::ExprStruct) {
        self.check_node(RustNode::ExprStruct, node.span());
        visit::visit_expr_struct(self, node);
    }

    fn visit_expr_try(&mut self, node: &syn::ExprTry) {
        self.check_node(RustNode::ExprTry, node.span());
        visit::visit_expr_try(self, node);
    }

    fn visit_expr_try_block(&mut self, node: &syn::ExprTryBlock) {
        self.check_node(RustNode::ExprTryBlock, node.span());
        visit::visit_expr_try_block(self, node);
    }

    fn visit_expr_tuple(&mut self, node: &syn::ExprTuple) {
        self.check_node(RustNode::ExprTuple, node.span());
        visit::visit_expr_tuple(self, node);
    }

    fn visit_expr_unary(&mut self, node: &syn::ExprUnary) {
        self.check_node(RustNode::ExprUnary, node.span());
        visit::visit_expr_unary(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &syn::ExprUnsafe) {
        self.check_node(RustNode::ExprUnsafe, node.span());
        visit::visit_expr_unsafe(self, node);
    }

    fn visit_expr_while(&mut self, node: &syn::ExprWhile) {
        self.check_node(RustNode::ExprWhile, node.span());
        visit::visit_expr_while(self, node);
    }

    fn visit_expr_yield(&mut self, node: &syn::ExprYield) {
        self.check_node(RustNode::ExprYield, node.span());
        visit::visit_expr_yield(self, node);
    }

    fn visit_field(&mut self, node: &syn::Field) {
        self.check_node(RustNode::Field, node.span());
        visit::visit_field(self, node);
    }

    fn visit_field_pat(&mut self, node: &syn::FieldPat) {
        self.check_node(RustNode::FieldPat, node.span());
        visit::visit_field_pat(self, node);
    }

    fn visit_field_value(&mut self, node: &syn::FieldValue) {
        self.check_node(RustNode::FieldValue, node.span());
        visit::visit_field_value(self, node);
    }

    fn visit_fields(&mut self, node: &syn::Fields) {
        self.check_node(RustNode::Fields, node.span());
        visit::visit_fields(self, node);
    }

    fn visit_fields_named(&mut self, node: &syn::FieldsNamed) {
        self.check_node(RustNode::FieldsNamed, node.span());
        visit::visit_fields_named(self, node);
    }

    fn visit_fields_unnamed(&mut self, node: &syn::FieldsUnnamed) {
        self.check_node(RustNode::FieldsUnnamed, node.span());
        visit::visit_fields_unnamed(self, node);
    }

    fn visit_file(&mut self, node: &syn::File) {
        self.check_node(RustNode::File, node.span());
        visit::visit_file(self, node);
    }

    fn visit_fn_arg(&mut self, node: &syn::FnArg) {
        self.check_node(RustNode::FnArg, node.span());
        visit::visit_fn_arg(self, node);
    }

    fn visit_foreign_item(&mut self, node: &syn::ForeignItem) {
        self.check_node(RustNode::ForeignItem, node.span());
        visit::visit_foreign_item(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &syn::ForeignItemFn) {
        self.check_node(RustNode::ForeignItemFn, node.span());
        visit::visit_foreign_item_fn(self, node);
    }

    fn visit_foreign_item_macro(&mut self, node: &syn::ForeignItemMacro) {
        self.check_node(RustNode::ForeignItemMacro, node.span());
        visit::visit_foreign_item_macro(self, node);
    }

    fn visit_foreign_item_static(&mut self, node: &syn::ForeignItemStatic) {
        self.check_node(RustNode::ForeignItemStatic, node.span());
        visit::visit_foreign_item_static(self, node);
    }

    fn visit_foreign_item_type(&mut self, node: &syn::ForeignItemType) {
        self.check_node(RustNode::ForeignItemType, node.span());
        visit::visit_foreign_item_type(self, node);
    }

    fn visit_generic_argument(&mut self, node: &syn::GenericArgument) {
        self.check_node(RustNode::GenericArgument, node.span());
        visit::visit_generic_argument(self, node);
    }

    fn visit_generic_param(&mut self, node: &syn::GenericParam) {
        self.check_node(RustNode::GenericParam, node.span());
        visit::visit_generic_param(self, node);
    }

    fn visit_generics(&mut self, node: &syn::Generics) {
        self.check_node(RustNode::Generics, node.span());
        visit::visit_generics(self, node);
    }

    fn visit_ident(&mut self, node: &proc_macro2::Ident) {
        self.check_node(RustNode::Ident, node.span());
        visit::visit_ident(self, node);
    }

    fn visit_impl_item(&mut self, node: &syn::ImplItem) {
        self.check_node(RustNode::ImplItem, node.span());
        visit::visit_impl_item(self, node);
    }

    fn visit_impl_item_const(&mut self, node: &syn::ImplItemConst) {
        self.check_node(RustNode::ImplItemConst, node.span());
        visit::visit_impl_item_const(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &syn::ImplItemFn) {
        self.check_node(RustNode::ImplItemFn, node.span());
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_impl_item_macro(&mut self, node: &syn::ImplItemMacro) {
        self.check_node(RustNode::ImplItemMacro, node.span());
        visit::visit_impl_item_macro(self, node);
    }

    fn visit_impl_item_type(&mut self, node: &syn::ImplItemType) {
        self.check_node(RustNode::ImplItemType, node.span());
        visit::visit_impl_item_type(self, node);
    }

    fn visit_index(&mut self, node: &syn::Index) {
        self.check_node(RustNode::Index, node.span());
        visit::visit_index(self, node);
    }

    fn visit_item(&mut self, node: &syn::Item) {
        self.check_node(RustNode::Item, node.span());
        visit::visit_item(self, node);
    }

    fn visit_item_const(&mut self, node: &syn::ItemConst) {
        self.check_node(RustNode::ItemConst, node.span());
        visit::visit_item_const(self, node);
    }

    fn visit_item_enum(&mut self, node: &syn::ItemEnum) {
        self.check_node(RustNode::ItemEnum, node.span());
        visit::visit_item_enum(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &syn::ItemExternCrate) {
        self.check_node(RustNode::ItemExternCrate, node.span());
        visit::visit_item_extern_crate(self, node);
    }

    fn visit_item_fn(&mut self, node: &syn::ItemFn) {
        self.check_node(RustNode::ItemFn, node.span());
        visit::visit_item_fn(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &syn::ItemForeignMod) {
        self.check_node(RustNode::ItemForeignMod, node.span());
        visit::visit_item_foreign_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
        self.check_node(RustNode::ItemImpl, node.span());
        visit::visit_item_impl(self, node);
    }

    fn visit_item_macro(&mut self, node: &syn::ItemMacro) {
        self.check_node(RustNode::ItemMacro, node.span());
        visit::visit_item_macro(self, node);
    }

    fn visit_item_mod(&mut self, node: &syn::ItemMod) {
        self.check_node(RustNode::ItemMod, node.span());
        visit::visit_item_mod(self, node);
    }

    fn visit_item_static(&mut self, node: &syn::ItemStatic) {
        self.check_node(RustNode::ItemStatic, node.span());
        visit::visit_item_static(self, node);
    }

    fn visit_item_struct(&mut self, node: &syn::ItemStruct) {
        self.check_node(RustNode::ItemStruct, node.span());
        visit::visit_item_struct(self, node);
    }

    fn visit_item_trait(&mut self, node: &syn::ItemTrait) {
        self.check_node(RustNode::ItemTrait, node.span());
        visit::visit_item_trait(self, node);
    }

    fn visit_item_trait_alias(&mut self, node: &syn::ItemTraitAlias) {
        self.check_node(RustNode::ItemTraitAlias, node.span());
        visit::visit_item_trait_alias(self, node);
    }

    fn visit_item_type(&mut self, node: &syn::ItemType) {
        self.check_node(RustNode::ItemType, node.span());
        visit::visit_item_type(self, node);
    }

    fn visit_item_union(&mut self, node: &syn::ItemUnion) {
        self.check_node(RustNode::ItemUnion, node.span());
        visit::visit_item_union(self, node);
    }

    fn visit_item_use(&mut self, node: &syn::ItemUse) {
        self.check_node(RustNode::ItemUse, node.span());
        let paths = self.collate_paths(&node.tree);
        paths
            .iter()
            .for_each(|path| self.check_path(path, node.span()));
        visit::visit_item_use(self, node);
    }

    fn visit_label(&mut self, node: &syn::Label) {
        self.check_node(RustNode::Label, node.span());
        visit::visit_label(self, node);
    }

    fn visit_lifetime(&mut self, node: &syn::Lifetime) {
        self.check_node(RustNode::Lifetime, node.span());
        visit::visit_lifetime(self, node);
    }

    fn visit_lifetime_param(&mut self, node: &syn::LifetimeParam) {
        self.check_node(RustNode::LifetimeParam, node.span());
        visit::visit_lifetime_param(self, node);
    }

    fn visit_lit(&mut self, node: &syn::Lit) {
        self.check_node(RustNode::Lit, node.span());
        visit::visit_lit(self, node);
    }

    fn visit_lit_bool(&mut self, node: &syn::LitBool) {
        self.check_node(RustNode::LitBool, node.span());
        visit::visit_lit_bool(self, node);
    }

    fn visit_lit_byte(&mut self, node: &syn::LitByte) {
        self.check_node(RustNode::LitByte, node.span());
        visit::visit_lit_byte(self, node);
    }

    fn visit_lit_byte_str(&mut self, node: &syn::LitByteStr) {
        self.check_node(RustNode::LitByteStr, node.span());
        visit::visit_lit_byte_str(self, node);
    }

    fn visit_lit_cstr(&mut self, node: &syn::LitCStr) {
        self.check_node(RustNode::LitCstr, node.span());
        visit::visit_lit_cstr(self, node);
    }

    fn visit_lit_char(&mut self, node: &syn::LitChar) {
        self.check_node(RustNode::LitChar, node.span());
        visit::visit_lit_char(self, node);
    }

    fn visit_lit_float(&mut self, node: &syn::LitFloat) {
        self.check_node(RustNode::LitFloat, node.span());
        visit::visit_lit_float(self, node);
    }

    fn visit_lit_int(&mut self, node: &syn::LitInt) {
        self.check_node(RustNode::LitInt, node.span());
        visit::visit_lit_int(self, node);
    }

    fn visit_lit_str(&mut self, node: &syn::LitStr) {
        self.check_node(RustNode::LitStr, node.span());
        visit::visit_lit_str(self, node);
    }

    fn visit_local(&mut self, node: &syn::Local) {
        self.check_node(RustNode::Local, node.span());
        visit::visit_local(self, node);
    }

    fn visit_local_init(&mut self, node: &syn::LocalInit) {
        self.check_node(
            RustNode::LocalInit,
            node.eq_token
                .span()
                .join(node.expr.span())
                .unwrap_or(node.eq_token.span()),
        );
        visit::visit_local_init(self, node);
    }

    fn visit_macro(&mut self, node: &syn::Macro) {
        self.check_node(RustNode::Macro, node.span());
        visit::visit_macro(self, node);
    }

    fn visit_macro_delimiter(&mut self, node: &syn::MacroDelimiter) {
        self.check_node(RustNode::MacroDelimiter, *node.span());
        visit::visit_macro_delimiter(self, node);
    }

    fn visit_member(&mut self, node: &syn::Member) {
        self.check_node(RustNode::Member, node.span());
        visit::visit_member(self, node);
    }

    fn visit_meta(&mut self, node: &syn::Meta) {
        self.check_node(RustNode::Meta, node.span());
        visit::visit_meta(self, node);
    }

    fn visit_meta_list(&mut self, node: &syn::MetaList) {
        self.check_node(RustNode::MetaList, node.span());
        visit::visit_meta_list(self, node);
    }

    fn visit_meta_name_value(&mut self, node: &syn::MetaNameValue) {
        self.check_node(RustNode::MetaNameValue, node.span());
        visit::visit_meta_name_value(self, node);
    }

    fn visit_parenthesized_generic_arguments(&mut self, node: &syn::ParenthesizedGenericArguments) {
        self.check_node(RustNode::ParenthesizedGenericArguments, node.span());
        visit::visit_parenthesized_generic_arguments(self, node);
    }

    fn visit_pat(&mut self, node: &syn::Pat) {
        self.check_node(RustNode::Pat, node.span());
        visit::visit_pat(self, node);
    }

    fn visit_pat_ident(&mut self, node: &syn::PatIdent) {
        self.check_node(RustNode::PatIdent, node.span());
        visit::visit_pat_ident(self, node);
    }

    fn visit_pat_or(&mut self, node: &syn::PatOr) {
        self.check_node(RustNode::PatOr, node.span());
        visit::visit_pat_or(self, node);
    }

    fn visit_pat_paren(&mut self, node: &syn::PatParen) {
        self.check_node(RustNode::PatParen, node.span());
        visit::visit_pat_paren(self, node);
    }

    fn visit_pat_reference(&mut self, node: &syn::PatReference) {
        self.check_node(RustNode::PatReference, node.span());
        visit::visit_pat_reference(self, node);
    }

    fn visit_pat_rest(&mut self, node: &syn::PatRest) {
        self.check_node(RustNode::PatRest, node.span());
        visit::visit_pat_rest(self, node);
    }

    fn visit_pat_slice(&mut self, node: &syn::PatSlice) {
        self.check_node(RustNode::PatSlice, node.span());
        visit::visit_pat_slice(self, node);
    }

    fn visit_pat_struct(&mut self, node: &syn::PatStruct) {
        self.check_node(RustNode::PatStruct, node.span());
        visit::visit_pat_struct(self, node);
    }

    fn visit_pat_tuple(&mut self, node: &syn::PatTuple) {
        self.check_node(RustNode::PatTuple, node.span());
        visit::visit_pat_tuple(self, node);
    }

    fn visit_pat_tuple_struct(&mut self, node: &syn::PatTupleStruct) {
        self.check_node(RustNode::PatTupleStruct, node.span());
        visit::visit_pat_tuple_struct(self, node);
    }

    fn visit_pat_type(&mut self, node: &syn::PatType) {
        self.check_node(RustNode::PatType, node.span());
        visit::visit_pat_type(self, node);
    }

    fn visit_pat_wild(&mut self, node: &syn::PatWild) {
        self.check_node(RustNode::PatWild, node.span());
        visit::visit_pat_wild(self, node);
    }

    fn visit_path(&mut self, node: &syn::Path) {
        self.check_node(RustNode::Path, node.span());
        let path = node
            .segments
            .to_token_stream()
            .into_iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
            .join("");
        self.check_path(&path, node.span());
        visit::visit_path(self, node);
    }

    fn visit_path_arguments(&mut self, node: &syn::PathArguments) {
        self.check_node(RustNode::PathArguments, node.span());
        visit::visit_path_arguments(self, node);
    }

    fn visit_path_segment(&mut self, node: &syn::PathSegment) {
        self.check_node(RustNode::PathSegment, node.span());
        visit::visit_path_segment(self, node);
    }

    fn visit_pointer_mutability(&mut self, node: &syn::PointerMutability) {
        self.check_node(RustNode::PointerMutability, node.span());
        visit::visit_pointer_mutability(self, node);
    }

    fn visit_precise_capture(&mut self, node: &syn::PreciseCapture) {
        self.check_node(RustNode::PreciseCapture, node.span());
        visit::visit_precise_capture(self, node);
    }

    fn visit_predicate_lifetime(&mut self, node: &syn::PredicateLifetime) {
        self.check_node(RustNode::PredicateLifetime, node.span());
        visit::visit_predicate_lifetime(self, node);
    }

    fn visit_predicate_type(&mut self, node: &syn::PredicateType) {
        self.check_node(RustNode::PredicateType, node.span());
        visit::visit_predicate_type(self, node);
    }

    fn visit_qself(&mut self, node: &syn::QSelf) {
        self.check_node(RustNode::Qself, node.span());
        visit::visit_qself(self, node);
    }

    fn visit_range_limits(&mut self, node: &syn::RangeLimits) {
        self.check_node(RustNode::RangeLimits, node.span());
        visit::visit_range_limits(self, node);
    }

    fn visit_receiver(&mut self, node: &syn::Receiver) {
        self.check_node(RustNode::Receiver, node.span());
        visit::visit_receiver(self, node);
    }

    fn visit_return_type(&mut self, node: &syn::ReturnType) {
        self.check_node(RustNode::ReturnType, node.span());
        visit::visit_return_type(self, node);
    }

    fn visit_signature(&mut self, node: &syn::Signature) {
        self.check_node(RustNode::Signature, node.span());
        visit::visit_signature(self, node);
    }

    fn visit_static_mutability(&mut self, node: &syn::StaticMutability) {
        self.check_node(RustNode::StaticMutability, node.span());
        visit::visit_static_mutability(self, node);
    }

    fn visit_stmt(&mut self, node: &syn::Stmt) {
        self.check_node(RustNode::Stmt, node.span());
        visit::visit_stmt(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &syn::StmtMacro) {
        self.check_node(RustNode::StmtMacro, node.span());
        visit::visit_stmt_macro(self, node);
    }

    fn visit_token_stream(&mut self, _node: &proc_macro2::TokenStream) {}

    fn visit_trait_bound(&mut self, node: &syn::TraitBound) {
        self.check_node(RustNode::TraitBound, node.span());
        visit::visit_trait_bound(self, node);
    }

    fn visit_trait_bound_modifier(&mut self, node: &syn::TraitBoundModifier) {
        self.check_node(RustNode::TraitBoundModifier, node.span());
        visit::visit_trait_bound_modifier(self, node);
    }

    fn visit_trait_item(&mut self, node: &syn::TraitItem) {
        self.check_node(RustNode::TraitItem, node.span());
        visit::visit_trait_item(self, node);
    }

    fn visit_trait_item_const(&mut self, node: &syn::TraitItemConst) {
        self.check_node(RustNode::TraitItemConst, node.span());
        visit::visit_trait_item_const(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &syn::TraitItemFn) {
        self.check_node(RustNode::TraitItemFn, node.span());
        visit::visit_trait_item_fn(self, node);
    }

    fn visit_trait_item_macro(&mut self, node: &syn::TraitItemMacro) {
        self.check_node(RustNode::TraitItemMacro, node.span());
        visit::visit_trait_item_macro(self, node);
    }

    fn visit_trait_item_type(&mut self, node: &syn::TraitItemType) {
        self.check_node(RustNode::TraitItemType, node.span());
        visit::visit_trait_item_type(self, node);
    }

    fn visit_type(&mut self, node: &syn::Type) {
        self.check_node(RustNode::Type, node.span());
        visit::visit_type(self, node);
    }

    fn visit_type_array(&mut self, node: &syn::TypeArray) {
        self.check_node(RustNode::TypeArray, node.span());
        visit::visit_type_array(self, node);
    }

    fn visit_type_bare_fn(&mut self, node: &syn::TypeBareFn) {
        self.check_node(RustNode::TypeBareFn, node.span());
        visit::visit_type_bare_fn(self, node);
    }

    fn visit_type_group(&mut self, node: &syn::TypeGroup) {
        self.check_node(RustNode::TypeGroup, node.span());
        visit::visit_type_group(self, node);
    }

    fn visit_type_impl_trait(&mut self, node: &syn::TypeImplTrait) {
        self.check_node(RustNode::TypeImplTrait, node.span());
        visit::visit_type_impl_trait(self, node);
    }

    fn visit_type_infer(&mut self, node: &syn::TypeInfer) {
        self.check_node(RustNode::TypeInfer, node.span());
        visit::visit_type_infer(self, node);
    }

    fn visit_type_macro(&mut self, node: &syn::TypeMacro) {
        self.check_node(RustNode::TypeMacro, node.span());
        visit::visit_type_macro(self, node);
    }

    fn visit_type_never(&mut self, node: &syn::TypeNever) {
        self.check_node(RustNode::TypeNever, node.span());
        visit::visit_type_never(self, node);
    }

    fn visit_type_param(&mut self, node: &syn::TypeParam) {
        self.check_node(RustNode::TypeParam, node.span());
        visit::visit_type_param(self, node);
    }

    fn visit_type_param_bound(&mut self, node: &syn::TypeParamBound) {
        self.check_node(RustNode::TypeParamBound, node.span());
        visit::visit_type_param_bound(self, node);
    }

    fn visit_type_paren(&mut self, node: &syn::TypeParen) {
        self.check_node(RustNode::TypeParen, node.span());
        visit::visit_type_paren(self, node);
    }

    fn visit_type_path(&mut self, node: &syn::TypePath) {
        self.check_node(RustNode::TypePath, node.span());
        visit::visit_type_path(self, node);
    }

    fn visit_type_ptr(&mut self, node: &syn::TypePtr) {
        self.check_node(RustNode::TypePtr, node.span());
        visit::visit_type_ptr(self, node);
    }

    fn visit_type_reference(&mut self, node: &syn::TypeReference) {
        self.check_node(RustNode::TypeReference, node.span());
        visit::visit_type_reference(self, node);
    }

    fn visit_type_slice(&mut self, node: &syn::TypeSlice) {
        self.check_node(RustNode::TypeSlice, node.span());
        visit::visit_type_slice(self, node);
    }

    fn visit_type_trait_object(&mut self, node: &syn::TypeTraitObject) {
        self.check_node(RustNode::TypeTraitObject, node.span());
        visit::visit_type_trait_object(self, node);
    }

    fn visit_type_tuple(&mut self, node: &syn::TypeTuple) {
        self.check_node(RustNode::TypeTuple, node.span());
        visit::visit_type_tuple(self, node);
    }

    fn visit_un_op(&mut self, node: &syn::UnOp) {
        self.check_node(RustNode::UnOp, node.span());
        visit::visit_un_op(self, node);
    }

    fn visit_use_glob(&mut self, node: &syn::UseGlob) {
        self.check_node(RustNode::UseGlob, node.span());
        visit::visit_use_glob(self, node);
    }

    fn visit_use_group(&mut self, node: &syn::UseGroup) {
        self.check_node(RustNode::UseGroup, node.span());
        visit::visit_use_group(self, node);
    }

    fn visit_use_name(&mut self, node: &syn::UseName) {
        self.check_node(RustNode::UseName, node.span());
        visit::visit_use_name(self, node);
    }

    fn visit_use_path(&mut self, node: &syn::UsePath) {
        self.check_node(RustNode::UsePath, node.span());
        visit::visit_use_path(self, node);
    }

    fn visit_use_rename(&mut self, node: &syn::UseRename) {
        self.check_node(RustNode::UseRename, node.span());
        visit::visit_use_rename(self, node);
    }

    fn visit_use_tree(&mut self, node: &syn::UseTree) {
        self.check_node(RustNode::UseTree, node.span());
        visit::visit_use_tree(self, node);
    }

    fn visit_variadic(&mut self, node: &syn::Variadic) {
        self.check_node(RustNode::Variadic, node.span());
        visit::visit_variadic(self, node);
    }

    fn visit_variant(&mut self, node: &syn::Variant) {
        self.check_node(RustNode::Variant, node.span());
        visit::visit_variant(self, node);
    }

    fn visit_vis_restricted(&mut self, node: &syn::VisRestricted) {
        self.check_node(RustNode::VisRestricted, node.span());
        visit::visit_vis_restricted(self, node);
    }

    fn visit_visibility(&mut self, node: &syn::Visibility) {
        self.check_node(RustNode::Visibility, node.span());
        visit::visit_visibility(self, node);
    }

    fn visit_where_clause(&mut self, node: &syn::WhereClause) {
        self.check_node(RustNode::WhereClause, node.span());
        visit::visit_where_clause(self, node);
    }

    fn visit_where_predicate(&mut self, node: &syn::WherePredicate) {
        self.check_node(RustNode::WherePredicate, node.span());
        visit::visit_where_predicate(self, node);
    }
}
