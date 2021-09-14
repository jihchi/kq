use kdl::KdlValue;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::combinator::{iterator, map, opt, value};
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::convert::TryFrom;

use crate::kdlrs;

#[derive(Debug, PartialEq)]
pub(crate) enum Combinator {
    Child(Accessor, Vec<(Sibling, Accessor)>),
    Descendant(Accessor, Vec<(Sibling, Accessor)>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Sibling {
    Adjacent,
    General,
}

impl TryFrom<&ParsedCombinator> for Sibling {
    type Error = &'static str;

    fn try_from(value: &ParsedCombinator) -> Result<Self, Self::Error> {
        match value {
            ParsedCombinator::Child => Err("Child can not convert to Sibling"),
            ParsedCombinator::Descendant => Err("Descendant can not convert to Sibling"),
            ParsedCombinator::GeneralSibling => Ok(Sibling::General),
            ParsedCombinator::AdjacentSibling => Ok(Sibling::Adjacent),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Accessor {
    AnyElement,
    AnyElementWithTypeTag(Option<String>),
    Closed(Option<String>, Matcher),
    Sole(String),
    Top,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Matcher {
    Direct(Entity),
    Expression(Entity, Operator, KdlValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    NodeName,
    PropName(String),
    Props,
    TypeTag,
    Val(usize),
    Values,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Contains,
    EndsWith,
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    NotEqual,
    StartsWith,
}

#[derive(Debug, Clone, PartialEq)]
enum ParsedCombinator {
    AdjacentSibling,
    Child,
    Descendant,
    GeneralSibling,
}

pub(crate) fn selector(input: &str) -> IResult<&str, Vec<Combinator>> {
    let (input, head) = accessor(input)?;
    let mut it = iterator(input, tuple((combinator, accessor)));
    let tail = it.collect::<Vec<(ParsedCombinator, Accessor)>>();
    it.finish()?;

    let mut output = vec![Combinator::Descendant(head, vec![])];
    let mut iter = tail.iter();
    let mut it = iter.next();

    while it.is_some() {
        // push sibling combinator onto last hierarchy combinator as a sibling
        while it.is_some() && is_sibling(it) {
            let (combinator, accessor) = it.unwrap();
            let combinator = Sibling::try_from(combinator).unwrap();
            let sibling = (combinator, accessor.clone());
            match output.last_mut().unwrap() {
                Combinator::Child(_head, siblings) | Combinator::Descendant(_head, siblings) => {
                    siblings.push(sibling)
                }
            };
            it = iter.next();
        }
        // push hierarchy combinator onto output
        if let Some((combinator, accessor)) = it {
            match combinator {
                ParsedCombinator::Child => output.push(Combinator::Child(accessor.clone(), vec![])),
                ParsedCombinator::Descendant => {
                    output.push(Combinator::Descendant(accessor.clone(), vec![]));
                }
                ParsedCombinator::GeneralSibling | ParsedCombinator::AdjacentSibling => (),
            };
            it = iter.next();
        }
    }

    Ok(("", output))
}

fn is_sibling(value: Option<&(ParsedCombinator, Accessor)>) -> bool {
    match value {
        Some((combinator, _accessor)) => match combinator {
            ParsedCombinator::Child | ParsedCombinator::Descendant => false,
            ParsedCombinator::AdjacentSibling | ParsedCombinator::GeneralSibling => true,
        },
        None => false,
    }
}

fn combinator(input: &str) -> IResult<&str, ParsedCombinator> {
    alt((
        delimited(
            many1(kdlrs::whitespace),
            alt((
                value(ParsedCombinator::Child, tag(">")),
                value(ParsedCombinator::AdjacentSibling, tag("+")),
                value(ParsedCombinator::GeneralSibling, tag("~")),
            )),
            many1(kdlrs::whitespace),
        ),
        value(ParsedCombinator::Descendant, many1(kdlrs::whitespace)),
    ))(input)
}

/// ```text
/// accessor :=
///   'top()' |
///   '[]' |
///   '(' identifier? ')' |
///   identifier? matcher |
///   identifier
/// ```
fn accessor(input: &str) -> IResult<&str, Accessor> {
    alt((
        value(Accessor::Top, tag("top()")),
        value(Accessor::AnyElement, tag("[]")),
        map(
            delimited(tag("("), opt(kdlrs::identifier), tag(")")),
            Accessor::AnyElementWithTypeTag,
        ),
        map(
            tuple((opt(kdlrs::identifier), matcher)),
            |(identifier, matcher)| Accessor::Closed(identifier, matcher),
        ),
        map(kdlrs::identifier, Accessor::Sole),
    ))(input)
}

/// `matcher := '[' entity (ws+ operator ws+ kdl-value)? ']'`
fn matcher(input: &str) -> IResult<&str, Matcher> {
    let (input, _) = tag("[")(input)?;
    let (input, left_hand_side) = entity(input)?;
    let (input, expression) = opt(tuple((
        delimited(many1(kdlrs::whitespace), operator, many1(kdlrs::whitespace)),
        kdlrs::node_value,
    )))(input)?;
    let (input, _) = tag("]")(input)?;

    let output = match expression {
        Some((operator, right_hand_side)) => {
            Matcher::Expression(left_hand_side, operator, right_hand_side)
        }
        None => Matcher::Direct(left_hand_side),
    };

    Ok((input, output))
}

/// ```text
/// entity :=
///   'name()' |
///   'tag()' |
///   'props()' |
///   'values()' |
///   'val(' digit* ')' |
///   'prop(' identifier ')' |
///   identifier '()'?
/// ```
fn entity(input: &str) -> IResult<&str, Entity> {
    alt((
        value(Entity::NodeName, tag("name()")),
        value(Entity::TypeTag, tag("tag()")),
        value(Entity::Props, tag("props()")),
        value(Entity::Values, tag("values()")),
        map(delimited(tag("val("), digit0, tag(")")), |input: &str| {
            Entity::Val(input.parse::<usize>().unwrap_or(0))
        }),
        map(
            delimited(tag("prop("), kdlrs::identifier, tag(")")),
            Entity::PropName,
        ),
        map(kdlrs::identifier, Entity::PropName),
    ))(input)
}

/// `operator := '=' | '!=' | '>' | '>=' | '<' | '<=' | '^=' | '$=' | '*='`
fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Contains, tag("*=")),
        value(Operator::EndsWith, tag("$=")),
        value(Operator::GreaterThanOrEqualTo, tag(">=")),
        value(Operator::LessThanOrEqualTo, tag("<=")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::StartsWith, tag("^=")),
        value(Operator::Equal, tag("=")),
        value(Operator::GreaterThan, tag(">")),
        value(Operator::LessThan, tag("<")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector() {
        assert_eq!(
            selector("top()"),
            Ok(("", vec![Combinator::Descendant(Accessor::Top, vec![]),]))
        );

        assert_eq!(
            selector("top() > []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(Accessor::Top, vec![]),
                    Combinator::Child(Accessor::AnyElement, vec![])
                ]
            ))
        );

        assert_eq!(
            selector("top() []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(Accessor::Top, vec![]),
                    Combinator::Descendant(Accessor::AnyElement, vec![])
                ]
            ))
        );

        assert_eq!(
            selector("a + b ~ c"),
            Ok((
                "",
                vec![Combinator::Descendant(
                    Accessor::Sole("a".to_owned()),
                    vec![
                        (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                        (Sibling::General, Accessor::Sole("c".to_owned()))
                    ]
                )]
            ))
        );

        assert_eq!(
            selector("a + b ~ c > []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Child(Accessor::AnyElement, vec![])
                ]
            ))
        );

        assert_eq!(
            selector("a + b ~ c []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Descendant(Accessor::AnyElement, vec![])
                ]
            ))
        );

        assert_eq!(
            selector("a + b ~ c d ~ e + f"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Descendant(
                        Accessor::Sole("d".to_owned()),
                        vec![
                            (Sibling::General, Accessor::Sole("e".to_owned())),
                            (Sibling::Adjacent, Accessor::Sole("f".to_owned()))
                        ]
                    ),
                ]
            ))
        );

        assert_eq!(
            selector("a + b ~ c > d ~ e + f"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Child(
                        Accessor::Sole("d".to_owned()),
                        vec![
                            (Sibling::General, Accessor::Sole("e".to_owned())),
                            (Sibling::Adjacent, Accessor::Sole("f".to_owned()))
                        ]
                    ),
                ]
            ))
        );

        assert_eq!(
            selector("top() a + b ~ c > []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(Accessor::Top, vec![]),
                    Combinator::Descendant(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Child(Accessor::AnyElement, vec![])
                ]
            ))
        );

        assert_eq!(
            selector("top() > a + b ~ c []"),
            Ok((
                "",
                vec![
                    Combinator::Descendant(Accessor::Top, vec![]),
                    Combinator::Child(
                        Accessor::Sole("a".to_owned()),
                        vec![
                            (Sibling::Adjacent, Accessor::Sole("b".to_owned())),
                            (Sibling::General, Accessor::Sole("c".to_owned()))
                        ]
                    ),
                    Combinator::Descendant(Accessor::AnyElement, vec![])
                ]
            ))
        );
    }

    #[test]
    fn test_combinator() {
        use super::ParsedCombinator::{AdjacentSibling, Child, Descendant, GeneralSibling};

        assert_eq!(combinator(" > "), Ok(("", Child)));
        assert_eq!(combinator(" + "), Ok(("", AdjacentSibling)));
        assert_eq!(combinator(" ~ "), Ok(("", GeneralSibling)));
        assert_eq!(combinator("   "), Ok(("", Descendant)));
    }

    #[test]
    fn test_accessor() {
        use super::Accessor::{AnyElement, Closed, Sole, Top};

        assert_eq!(accessor("[]"), Ok(("", AnyElement)));
        assert_eq!(accessor("name"), Ok(("", Sole("name".to_owned()))));
        assert_eq!(accessor("top()"), Ok(("", Top)));
        assert_eq!(
            accessor("[props()]"),
            Ok(("", Closed(None, Matcher::Direct(Entity::Props))))
        );
        assert_eq!(
            accessor("name[props()]"),
            Ok((
                "",
                Closed(Some("name".to_owned()), Matcher::Direct(Entity::Props))
            ))
        );
    }

    #[test]
    fn test_matcher() {
        use super::Matcher::{Direct, Expression};

        assert_eq!(matcher("[name()]"), Ok(("", Direct(Entity::NodeName))));
        assert_eq!(matcher("[tag()]"), Ok(("", Direct(Entity::TypeTag))));
        assert_eq!(matcher("[props()]"), Ok(("", Direct(Entity::Props))));
        assert_eq!(matcher("[values()]"), Ok(("", Direct(Entity::Values))));
        assert_eq!(matcher("[val()]"), Ok(("", Direct(Entity::Val(0)))));
        assert_eq!(matcher("[val(777)]"), Ok(("", Direct(Entity::Val(777)))));
        assert_eq!(
            matcher("[prop(name)]"),
            Ok(("", Direct(Entity::PropName("name".to_owned()))))
        );
        assert_eq!(
            matcher("[prop]"),
            Ok(("", Direct(Entity::PropName("prop".to_owned()))))
        );
        assert!(matcher("[some()]").is_err());

        assert_eq!(
            matcher(r#"[name() = "kdl"]"#),
            Ok((
                "",
                Expression(Entity::NodeName, Operator::Equal, "kdl".into())
            ))
        );
        assert_eq!(
            matcher(r#"[tag() = "kdl"]"#),
            Ok((
                "",
                Expression(Entity::TypeTag, Operator::Equal, "kdl".into())
            ))
        );
        assert_eq!(
            matcher(r#"[props() = "kdl"]"#),
            Ok(("", Expression(Entity::Props, Operator::Equal, "kdl".into())))
        );
        assert_eq!(
            matcher(r#"[values() = "kdl"]"#),
            Ok((
                "",
                Expression(Entity::Values, Operator::Equal, "kdl".into())
            ))
        );
        assert_eq!(
            matcher(r#"[val() = 777]"#),
            Ok(("", Expression(Entity::Val(0), Operator::Equal, 777.into())))
        );
        assert_eq!(
            matcher("[val(777) = 777]"),
            Ok((
                "",
                Expression(Entity::Val(777), Operator::Equal, 777.into())
            ))
        );
        assert_eq!(
            matcher("[prop(name) = 777]"),
            Ok((
                "",
                Expression(
                    Entity::PropName("name".to_owned()),
                    Operator::Equal,
                    777.into()
                )
            ))
        );
        assert_eq!(
            matcher("[prop = 777]"),
            Ok((
                "",
                Expression(
                    Entity::PropName("prop".to_owned()),
                    Operator::Equal,
                    777.into()
                )
            ))
        );
        assert!(matcher("[some() = 777]").is_err());
    }

    #[test]
    fn test_entity() {
        use super::Entity::{NodeName, PropName, Props, TypeTag, Val, Values};

        assert_eq!(entity("name()"), Ok(("", NodeName)));
        assert_eq!(entity("tag()"), Ok(("", TypeTag)));
        assert_eq!(entity("props()"), Ok(("", Props)));
        assert_eq!(entity("values()"), Ok(("", Values)));
        assert_eq!(entity("val()"), Ok(("", Val(0))));
        assert_eq!(entity("val(777)"), Ok(("", Val(777))));
        assert_eq!(
            entity("val(3.14)"),
            Ok(("(3.14)", PropName("val".to_owned())))
        );
        assert_eq!(entity("val(-0)"), Ok(("(-0)", PropName("val".to_owned()))));
        assert_eq!(entity("prop(name)"), Ok(("", PropName("name".to_owned()))));
        assert_eq!(entity("prop"), Ok(("", PropName("prop".to_owned()))));
        assert_eq!(entity("prop()"), Ok(("()", PropName("prop".to_owned()))));
        assert_eq!(entity("some()"), Ok(("()", PropName("some".to_owned()))));

        assert!(entity("0xEF").is_err());
    }

    #[test]
    fn test_operator() {
        use super::Operator::{
            Contains, EndsWith, Equal, GreaterThan, GreaterThanOrEqualTo, LessThan,
            LessThanOrEqualTo, NotEqual, StartsWith,
        };

        assert_eq!(operator("!="), Ok(("", NotEqual)));
        assert_eq!(operator("$="), Ok(("", EndsWith)));
        assert_eq!(operator("*="), Ok(("", Contains)));
        assert_eq!(operator("<="), Ok(("", LessThanOrEqualTo)));
        assert_eq!(operator(">="), Ok(("", GreaterThanOrEqualTo)));
        assert_eq!(operator("^="), Ok(("", StartsWith)));
        assert_eq!(operator("<"), Ok(("", LessThan)));
        assert_eq!(operator("="), Ok(("", Equal)));
        assert_eq!(operator(">"), Ok(("", GreaterThan)));

        assert!(operator("?").is_err());
        assert!(operator("?=").is_err());
        assert!(operator("?= ...").is_err());
    }
}
