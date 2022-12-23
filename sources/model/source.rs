use std::collections::BTreeMap;


#[derive(Debug)]
pub enum Source {
	Dynamic(String),
	Static(BTreeMap<usize, String>),
}
