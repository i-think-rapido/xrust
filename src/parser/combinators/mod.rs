pub(crate) mod alt;
pub(crate) mod delimited;
pub(crate) mod many;
pub(crate) mod map;
pub(crate) mod tag;
pub(crate) mod take;
pub(crate) mod tuple;

pub(crate) mod expander;
pub(crate) mod opt;
pub(crate) mod whitespace;
pub(crate) mod validate;
pub(crate) mod none_of;
pub(crate) mod value;

pub(crate) type ParseResult<Output> = Result<(String, usize, Output), usize>;