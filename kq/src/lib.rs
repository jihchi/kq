use kdl::{KdlNode, KdlValue};
use nom::combinator::all_consuming;
use nom::Finish;
use std::collections::VecDeque;
use std::iter;

mod evaluation;
mod kdlrs;
mod parser;

use parser::{Accessor, Combinator, Entity, Matcher, Operator, Sibling};

pub fn query_document(input: &str, document: Vec<KdlNode>) -> Result<Vec<KdlNode>, String> {
    let input = input.trim();
    if input.is_empty() {
        Ok(document)
    } else {
        all_consuming(parser::selector)(input)
            .finish()
            .map(|(_input, selector)| query_by_selector(selector, document))
            .map_err(|error| error.to_string())
    }
}

fn query_by_selector(selector: Vec<Combinator>, document: Vec<KdlNode>) -> Vec<KdlNode> {
    selector
        .iter()
        .fold(
            (&Accessor::Top, document),
            |(previous, document), combinator| match combinator {
                Combinator::Child(accessor, siblings) => {
                    let is_previous_sibling_top = match previous {
                        Accessor::AnyElement
                        | Accessor::AnyElementWithTypeTag(_)
                        | Accessor::Closed(_, _)
                        | Accessor::Sole(_) => false,
                        Accessor::Top => true,
                    };
                    let document = query_by_child_combinator(
                        is_previous_sibling_top,
                        accessor,
                        siblings,
                        document,
                    );
                    (accessor, document)
                }
                Combinator::Descendant(accessor, siblings) => {
                    let document = query_by_descendant_combinator(accessor, siblings, document);
                    (accessor, document)
                }
            },
        )
        .1
}

fn query_by_child_combinator(
    is_previous_sibling_top: bool,
    accessor: &Accessor,
    siblings: &[(Sibling, Accessor)],
    document: Vec<KdlNode>,
) -> Vec<KdlNode> {
    if siblings.is_empty() {
        match accessor {
            Accessor::AnyElement => {
                if is_previous_sibling_top {
                    document
                } else {
                    document
                        .iter()
                        .flat_map(|node| &node.children)
                        .cloned()
                        .collect()
                }
            }
            Accessor::AnyElementWithTypeTag(_identifier) => vec![],
            Accessor::Closed(identifier, matcher) => identifier
                .as_ref()
                .map(|identifier| filter_by_identifier(identifier, &document))
                .unwrap_or(document)
                .iter()
                .filter(|node| match_by_matcher(matcher, node))
                .cloned()
                .collect(),
            Accessor::Sole(identifier) => {
                if is_previous_sibling_top {
                    filter_by_identifier(identifier, &document)
                } else {
                    document
                        .iter()
                        .flat_map(|node| filter_by_identifier(identifier, &node.children))
                        .collect()
                }
            }
            Accessor::Top => document,
        }
    } else if is_previous_sibling_top {
        filter_by_siblings(accessor, siblings, &document)
    } else {
        document
            .iter()
            .flat_map(|node| filter_by_siblings(accessor, siblings, &node.children))
            .collect()
    }
}

fn query_by_descendant_combinator(
    accessor: &Accessor,
    siblings: &[(Sibling, Accessor)],
    document: Vec<KdlNode>,
) -> Vec<KdlNode> {
    if siblings.is_empty() {
        match accessor {
            Accessor::AnyElement => document,
            Accessor::AnyElementWithTypeTag(_identifier) => vec![],
            Accessor::Closed(identifier, matcher) => traverse(
                |node| match_by_accessor_filter(identifier, matcher, node),
                &document,
            ),
            Accessor::Sole(identifier) => traverse(|node| node.name == *identifier, &document),
            Accessor::Top => document,
        }
    } else {
        traverse_by_siblings(accessor, siblings, &document)
    }
}

fn filter_by_identifier(identifier: &str, document: &[KdlNode]) -> Vec<KdlNode> {
    document
        .iter()
        .filter(|node| node.name == *identifier)
        .cloned()
        .collect()
}

fn filter_by_siblings(
    accessor: &Accessor,
    siblings: &[(Sibling, Accessor)],
    document: &[KdlNode],
) -> Vec<KdlNode> {
    let head = (Sibling::General, accessor.clone());

    document
        .iter()
        .enumerate()
        .filter(|(i, node)| {
            let mut siblings = iter::once(&head).chain(siblings.iter()).rev();
            let mut preceding = document[..*i].iter().rev().peekable();

            let result = siblings
                .next()
                .and_then(|(sibling, accessor)| match_by_accessor(accessor, node).then(|| sibling));

            let result = result.map(|sibling| {
                let mut previous_sibling = sibling;

                siblings.all(|(sibling, accessor)| {
                    let is_sibling_matched = match previous_sibling {
                        Sibling::Adjacent => preceding
                            .next()
                            .map(|node| match_by_accessor(accessor, node))
                            .unwrap_or(false),
                        Sibling::General => preceding.any(|node| match_by_accessor(accessor, node)),
                    };
                    previous_sibling = sibling;
                    is_sibling_matched
                })
            });

            result.unwrap_or(false)
        })
        .map(|(_i, node)| node)
        .cloned()
        .collect()
}

fn match_by_matcher(matcher: &Matcher, node: &KdlNode) -> bool {
    match matcher {
        Matcher::Direct(entity) => match entity {
            Entity::PropName(name) => node.properties.contains_key(name),
            Entity::Val(index) => node.values.len() > *index,
            // '[name()]', '[props()]',, and '[values()]' does not make sense by themselves in a matcher
            // '[tag()]' is unsupported
            Entity::NodeName | Entity::Props | Entity::TypeTag | Entity::Values => false,
        },
        Matcher::Expression(entity, operator, value) => match entity {
            Entity::PropName(name) => node
                .properties
                .get(name)
                .map(|lhs| evaluation::evaluate(lhs, operator, value))
                .unwrap_or(false),
            Entity::Val(index) => node
                .values
                .get(*index)
                .map(|lhs| evaluation::evaluate(lhs, operator, value))
                .unwrap_or(false),
            Entity::NodeName => match value {
                KdlValue::String(string) => match operator {
                    Operator::Contains => node.name.contains(string),
                    Operator::EndsWith => node.name.ends_with(string),
                    Operator::Equal => &node.name == string,
                    Operator::GreaterThan => false,
                    Operator::GreaterThanOrEqualTo => false,
                    Operator::LessThan => false,
                    Operator::LessThanOrEqualTo => false,
                    Operator::NotEqual => &node.name != string,
                    Operator::StartsWith => node.name.starts_with(string),
                },
                KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                    false
                }
            },
            Entity::Props => false,
            Entity::TypeTag => false,
            Entity::Values => false,
        },
    }
}

fn match_by_accessor(accessor: &Accessor, node: &KdlNode) -> bool {
    match accessor {
        Accessor::AnyElement => true,
        Accessor::AnyElementWithTypeTag(_identifier) => false,
        Accessor::Closed(identifier, matcher) => {
            match_by_accessor_filter(identifier, matcher, node)
        }
        Accessor::Sole(identifier) => node.name == *identifier,
        Accessor::Top => true,
    }
}

fn match_by_accessor_filter(
    identifier: &Option<String>,
    matcher: &Matcher,
    node: &KdlNode,
) -> bool {
    identifier
        .as_ref()
        .map(|identifier| node.name == *identifier)
        .unwrap_or(true)
        .then(|| match_by_matcher(matcher, node))
        .unwrap_or(false)
}

fn traverse_by_siblings(
    accessor: &Accessor,
    siblings: &[(Sibling, Accessor)],
    document: &[KdlNode],
) -> Vec<KdlNode> {
    let mut result = Vec::<KdlNode>::new();
    let mut queue = VecDeque::<&[KdlNode]>::new();
    queue.push_back(document);

    while let Some(document) = queue.pop_front() {
        for node in filter_by_siblings(accessor, siblings, document) {
            result.push(node);
        }
        for node in document {
            if !node.children.is_empty() {
                queue.push_back(&node.children);
            }
        }
    }

    result
}

fn traverse<F>(predicate: F, document: &[KdlNode]) -> Vec<KdlNode>
where
    F: Fn(&KdlNode) -> bool,
{
    let mut result: Vec<KdlNode> = vec![];
    let mut queue: VecDeque<&KdlNode> = document.iter().collect();

    while let Some(node) = queue.pop_front() {
        if predicate(node) {
            result.push(node.clone());
        }
        queue.extend(node.children.iter());
    }

    result
}
