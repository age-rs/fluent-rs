//! The `FluentValue` enum represents values which can be formatted to a String.
//!
//! The [`ResolveValue`][] trait from the [`resolve`][] module evaluates AST nodes into
//! `FluentValues` which can then be formatted to Strings using the i18n formatters stored by the
//! `MessageContext` instance if required.
//!
//! The arguments `HashMap` passed to [`MessageContext::format`][] should also use `FluentValues`
//! as values of arguments.
//!
//! [`ResolveValue`]: ../resolve/trait.ResolveValue.html
//! [`resolve`]: ../resolve
//! [`MessageContext::format`]: ../context/struct.MessageContext.html#method.format

extern crate fluent_locale;
use self::fluent_locale::{negotiate_languages, NegotiationStrategy};

extern crate intl_pluralrules;
use self::intl_pluralrules::{IntlPluralRules, PluralCategory, PluralRuleType};

use std::f32;
use std::num::ParseFloatError;
use std::str::FromStr;

use super::context::MessageContext;

/// Value types which can be formatted to a String.
#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    /// Fluent String type.
    String(String),
    /// Fluent Number type.
    Number(String),
}

impl FluentValue {
    pub fn as_number<S: ToString>(v: S) -> Result<Self, ParseFloatError> {
        f64::from_str(&v.to_string()).map(|_| FluentValue::Number(v.to_string()))
    }

    pub fn format(&self, _ctx: &MessageContext) -> String {
        match self {
            FluentValue::String(s) => s.clone(),
            FluentValue::Number(n) => n.clone(),
        }
    }

    pub fn matches(&self, ctx: &MessageContext, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => a == b,
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                println!("Number: {:#?}", b);
                //XXX: This is a dirty hack and should be replaced with a
                //lazy resolved cache on the context.
                let cat = match a.as_str() {
                    "zero" => PluralCategory::ZERO,
                    "one" => PluralCategory::ONE,
                    "two" => PluralCategory::TWO,
                    "few" => PluralCategory::FEW,
                    "many" => PluralCategory::MANY,
                    "other" => PluralCategory::OTHER,
                    _ => return false,
                };

                let locales = negotiate_languages(
                    ctx.locales,
                    IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
                    Some("en"),
                    &NegotiationStrategy::Lookup,
                );

                let pr = IntlPluralRules::create(locales[0], PluralRuleType::CARDINAL).unwrap();
                pr.select(&b) == Ok(cat)
            }
            (&FluentValue::Number(..), &FluentValue::String(..)) => false,
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl<'a> From<&'a str> for FluentValue {
    fn from(s: &'a str) -> Self {
        FluentValue::String(String::from(s))
    }
}

impl From<f32> for FluentValue {
    fn from(n: f32) -> Self {
        FluentValue::Number(n.to_string())
    }
}

impl From<i8> for FluentValue {
    fn from(n: i8) -> Self {
        FluentValue::Number(n.to_string())
    }
}
