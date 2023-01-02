use std::collections::BTreeMap;


#[derive(Clone, Debug)]
pub enum Source {
	Dynamic(String),
	Static(BTreeMap<usize, String>),
}
