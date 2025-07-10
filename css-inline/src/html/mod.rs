mod attributes;
mod document;
mod element;
mod iter;
pub(crate) mod node;
mod parser;
mod selectors;
mod serializer;

pub(crate) use self::selectors::Specificity;
use crate::hasher::BuildNoHashHasher;
pub(crate) use document::Document;
pub(crate) use element::Element;
use indexmap::IndexMap;
pub(crate) use parser::InliningMode;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

pub(crate) type ElementStyleMap<'i> =
    IndexMap<&'i str, (Specificity, &'i str), BuildHasherDefault<FxHasher>>;

pub(crate) type DocumentStyleMap<'i> =
    IndexMap<node::NodeId, ElementStyleMap<'i>, BuildNoHashHasher>;
