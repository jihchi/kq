// The code of this file is modified from https://github.com/kdl-org/kdl-rs
// (mainly from https://github.com/kdl-org/kdl-rs/blob/main/src/parser.rs)
use kdl::KdlValue;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_until1, take_while_m_n};
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{eof, map, map_opt, map_res, not, opt, recognize, value};
use nom::multi::{fold_many0, many0, many1, many_till};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L148-L151
/// `identifier := bare_identifier | string`
///
// fn identifier(input: &str) -> IResult<&str, String, KdlParseError<&str>> {
pub(crate) fn identifier(input: &str) -> IResult<&str, String> {
    alt((string, (map(bare_identifier, String::from))))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L124-L142
/// `bare_identifier := ((identifier-char - digit - sign) identifier-char* | sign ((identifier-char - digit) identifier-char*)?) - keyword`
///
// fn bare_identifier(input: &str) -> IResult<&str, &str, KdlParseError<&str>>> {
fn bare_identifier(input: &str) -> IResult<&str, &str> {
    // fn left(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    fn left(input: &str) -> IResult<&str, ()> {
        not(keyword)(input)?;
        not(one_of("0123456789"))(input)?;
        not(one_of("+-"))(input)?;
        let (input, _) = identifier_char(input)?;
        let (input, _) = many0(identifier_char)(input)?;
        Ok((input, ()))
    }
    // fn right(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    fn right(input: &str) -> IResult<&str, ()> {
        let (input, _) = one_of("+-")(input)?;
        not(keyword)(input)?;
        not(one_of("0123456789"))(input)?;
        let (input, _) = opt(many1(identifier_char))(input)?;
        Ok((input, ()))
    }
    recognize(alt((left, right)))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L144-L146
/// `string := '"' character* '"'`
///
// fn keyword(input: &str) -> IResult<&str, String, KdlParseError<&str>> {
fn keyword(input: &str) -> IResult<&str, String> {
    map(alt((tag("true"), tag("false"), tag("null"))), String::from)(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L118-L122
/// `identifier_char := unicode - linespace - [\/(){}<>;[]=,"]
///
// fn identifier_char(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
fn identifier_char(input: &str) -> IResult<&str, &str> {
    not(linespace)(input)?;
    recognize(none_of(r#"\/(){}<>;[]=,""#))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L118-L122
/// `linespace := newline | ws | single-line-comment`
///
// fn linespace(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
fn linespace(input: &str) -> IResult<&str, ()> {
    value((), alt((newline, whitespace, single_line_comment)))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L473-L487
/// `newline := All line-break unicode white_space
///
// fn newline(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
fn newline(input: &str) -> IResult<&str, ()> {
    value(
        (),
        alt((
            tag("\r\n"),
            tag("\r"),
            tag("\n"),
            tag("\u{0085}"),
            tag("\u{000C}"),
            tag("\u{2028}"),
            tag("\u{2029}"),
        )),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L437-L448
/// `ws := bom | unicode-space | multi-line-comment`
///
// fn whitespace(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
pub(crate) fn whitespace(input: &str) -> IResult<&str, ()> {
    // TODO: bom?
    value(
        (),
        alt((
            tag("\u{FEFF}"),
            unicode_space,
            recognize(multi_line_comment),
        )),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L400-L405
/// `single-line-comment := '//' ('\r' [^\n] | [^\r\n])* (newline | eof)`
///
// fn single_line_comment(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
fn single_line_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = many_till(value((), anychar), alt((newline, value((), eof))))(input)?;
    Ok((input, ()))
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L407-L411
/// `multi-line-comment := '/*' commented-block
///
// fn multi_line_comment(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
fn multi_line_comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("/*")(input)?;
    commented_block(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L413-L422
/// `commented-block := '*/' | (multi-line-comment | '*' | '/' | [^*/]+) commented-block`
///
// fn commented_block(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
fn commented_block(input: &str) -> IResult<&str, &str> {
    alt((
        tag("*/"),
        terminated(
            alt((multi_line_comment, take_until1("*/"), tag("*"), tag("/"))),
            commented_block,
        ),
    ))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L450-L471
///
// fn unicode_space(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
fn unicode_space(input: &str) -> IResult<&str, &str> {
    alt((
        tag(" "),
        tag("\t"),
        tag("\u{00A0}"),
        tag("\u{1680}"),
        tag("\u{2000}"),
        tag("\u{2001}"),
        tag("\u{2002}"),
        tag("\u{2003}"),
        tag("\u{2004}"),
        tag("\u{2005}"),
        tag("\u{2006}"),
        tag("\u{2007}"),
        tag("\u{2008}"),
        tag("\u{2009}"),
        tag("\u{200A}"),
        tag("\u{202F}"),
        tag("\u{205F}"),
        tag("\u{3000}"),
    ))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L213-L223
/// `string := '"' character* '"'`
///
// fn string(input: &str) -> IResult<&str, String, KdlParseError<&str>> {
fn string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        fold_many0(character, String::new, |mut acc, ch| {
            acc.push(ch);
            acc
        }),
        char('"'),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L225-L228
/// `character := '\' escape | [^\"]`
///
// fn character(input: &str) -> IResult<&str, char, KdlParseError<&str>> {
fn character(input: &str) -> IResult<&str, char> {
    alt((preceded(char('\\'), escape), none_of("\\\"")))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L237-L247
/// a map and its inverse of escape-sequence<->char
///
/// (instead of building a map by phf, use a function with pattern matching)
fn escape_chars(input: char) -> Option<char> {
    match input {
        '"' => Some('"'),
        '\\' => Some('\\'),
        '/' => Some('/'),
        'b' => Some('\u{08}'),
        'f' => Some('\u{0C}'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        _ => None,
    }
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L249-L255
/// `escape := ["\\/bfnrt] | 'u{' hex-digit{1, 6} '}'`
///
// fn escape(input: &str) -> IResult<&str, char, KdlParseError<&str>> {
fn escape(input: &str) -> IResult<&str, char> {
    alt((
        delimited(tag("u{"), unicode, char('}')),
        map_opt(anychar, escape_chars),
    ))(input)
}

// fn unicode(input: &str) -> IResult<&str, char, KdlParseError<&str>> {
fn unicode(input: &str) -> IResult<&str, char> {
    map_opt(
        map_res(
            take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
            |hex| u32::from_str_radix(hex, 16),
        ),
        std::char::from_u32,
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L180-L190
/// `value := type-annotation? (string | raw_string | number | boolean | 'null'`)
///
// fn node_value(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
pub(crate) fn node_value(input: &str) -> IResult<&str, KdlValue> {
    // let (input, _ty) = opt(type_annotation)(input)?;
    alt((
        map(string, KdlValue::String),
        map(raw_string, |s| KdlValue::String(s.into())),
        number,
        boolean,
        value(KdlValue::Null, tag("null")),
    ))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L267-L278
/// `raw-string := 'r' raw-string-hash`
/// `raw-string-hash := '#' raw-string-hash '#' | raw-string-quotes`
/// `raw-string-quotes := '"' .* '"'`
///
// fn raw_string(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
fn raw_string(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('r')(input)?;
    let (input, hashes) = recognize(many0(char('#')))(input)?;
    let (input, _) = char('"')(input)?;
    let close = format!("\"{}", hashes);
    let (input, string) = take_until(&close[..])(input)?;
    let (input, _) = tag(&close[..])(input)?;
    Ok((input, string))
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L280-L289
/// `number := decimal | hex | octal | binary`
///
// fn number(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
fn number(input: &str) -> IResult<&str, KdlValue> {
    alt((
        map(hexadecimal, KdlValue::Int),
        map(octal, KdlValue::Int),
        map(binary, KdlValue::Int),
        map(float, KdlValue::Float),
        map(integer, KdlValue::Int),
    ))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L331-L343
///
// fn sign(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
fn sign(input: &str) -> IResult<&str, i64> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let mult = if let Some(sign) = sign {
        if sign == '+' {
            1
        } else {
            -1
        }
    } else {
        1
    };
    Ok((input, mult))
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L345-L358
/// `hex := sign? '0x' [0-9a-fA-F] [0-9a-fA-F_]*`
///
// fn hexadecimal(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
fn hexadecimal(input: &str) -> IResult<&str, i64> {
    let (input, sign) = sign(input)?;
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(out, "_", ""), 16).map(|x| x * sign),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L360-L370
///
/// `octal := sign? '0o' [0-7] [0-7_]*`
// fn octal(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
fn octal(input: &str) -> IResult<&str, i64> {
    let (input, sign) = sign(input)?;
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(out, "_", ""), 8).map(|x| x * sign),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L372-L382
///
// fn binary(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
fn binary(input: &str) -> IResult<&str, i64> {
    let (input, sign) = sign(input)?;
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(out, "_", ""), 2).map(|x| x * sign),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L384-L390
/// `boolean := 'true' | 'false'`
///
// fn boolean(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
fn boolean(input: &str) -> IResult<&str, KdlValue> {
    alt((
        value(KdlValue::Boolean(true), tag("true")),
        value(KdlValue::Boolean(false), tag("false")),
    ))(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L291-L311
/// ```text
/// decimal := integer ('.' [0-9]+)? exponent?
/// exponent := ('e' | 'E') integer
/// integer := sign? [1-9] [0-9_]*
/// sign := '+' | '-'
/// ```
///
// fn float(input: &str) -> IResult<&str, f64, KdlParseError<&str>> {
fn float(input: &str) -> IResult<&str, f64> {
    map_res(
        alt((
            recognize(tuple((
                integer,
                opt(preceded(char('.'), integer)),
                one_of("eE"),
                opt(one_of("+-")),
                integer,
            ))),
            recognize(tuple((integer, char('.'), integer))),
        )),
        |x| str::replace(x, "_", "").parse::<f64>(),
    )(input)
}

/// https://github.com/kdl-org/kdl-rs/blob/v3.0.0/src/parser.rs#L313-L329
/// ```text
/// decimal := integer ('.' [0-9]+)? exponent?
/// exponent := ('e' | 'E') integer
/// integer := sign? [1-9] [0-9_]*
/// sign := '+' | '-'
/// ```
///
// fn integer(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
fn integer(input: &str) -> IResult<&str, i64> {
    let (input, sign) = sign(input)?;
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        move |out: &str| {
            str::replace(out, "_", "")
                .parse::<i64>()
                .map(move |x| x * sign)
        },
    )(input)
}
