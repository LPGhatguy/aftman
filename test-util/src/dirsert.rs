use std::collections::{btree_map, BTreeMap};
use std::path::Path;

#[derive(Default)]
pub struct Dir {
    children: BTreeMap<String, Entry>,
    exhaustive: bool,
}

enum Entry {
    File { _contents: Option<String> },

    Dir(Dir),
}

impl Dir {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            exhaustive: false,
        }
    }

    pub fn file<N, C>(&mut self, name: N, contents: Option<C>)
    where
        N: Into<String>,
        C: Into<String>,
    {
        self.children.insert(
            name.into(),
            Entry::File {
                _contents: contents.map(Into::into),
            },
        );
    }

    pub fn dir<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<String>,
    {
        let name = name.into();

        let value = Entry::Dir(Dir::new());
        let value = match self.children.entry(name) {
            btree_map::Entry::Vacant(v) => v.insert(value),
            btree_map::Entry::Occupied(mut o) => {
                o.insert(value);
                o.into_mut()
            }
        };

        match value {
            Entry::Dir(dir) => dir,
            _ => unreachable!(),
        }
    }

    pub fn exhaustive(&mut self) -> &mut Self {
        self.exhaustive = true;
        self
    }

    pub fn assert(&self, _dir: &Path) {
        todo!()
    }
}
