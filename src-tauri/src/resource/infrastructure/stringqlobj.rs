
#[derive(Clone)]
pub enum StringQLGroupPrefix {
    Include,
    Exclude,
}

#[derive(Clone)]
pub struct StringQLGroup {
    pub prefix: StringQLGroupPrefix,
    pub items: Vec<String>,
}

/// Immutable String QL Object \
/// Using `StringQLObjectBuilder` to build object
#[derive(Clone)]
pub struct StringQLObject {
    excludes: Vec<String>,

    includes: Vec<String>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { excludes: Vec::new(), includes: Vec::new(), groups: Vec::new() }
    }

    pub fn get_excludes(&self) -> &Vec<String> {
        &self.excludes
    }

    pub fn get_includes(&self) -> &Vec<String>{
        &self.includes
    }

    pub fn get_groups(&self) ->  &Vec<StringQLGroup> {
        &self.groups
    }
    
}

pub struct StringQLObjectBuilder {
    obj: StringQLObject,
}

impl StringQLObjectBuilder {
    pub fn new() -> Self {
        Self { obj: StringQLObject::new() }
    }

    pub fn add_exclude(mut self, item: String) -> Self {
        self.obj.excludes.push(item);
        self
    }

    pub fn add_include(mut self, item: String) -> Self {
        self.obj.includes.push(item);
        self
    }

    pub fn add_group(mut self, prefix: StringQLGroupPrefix, items: Vec<String>) -> Self {
        self.obj.groups.push(StringQLGroup { prefix, items });
        self
    }

    pub fn build(&self) -> StringQLObject {
        self.obj.to_owned()
    }
}