use std::borrow::Cow;
use std::collections::HashMap;

pub mod users;

pub type ConduitErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;
