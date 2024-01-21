// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub enum StringQLGroupPrefix {
    Include,
    Exclude,
}

// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub struct StringQLTagItem {
    tag_id: String,

    attribute: Option<String>,
}
impl StringQLTagItem {
    pub fn new(tag_id: String, attribute: Option<String>) -> Self {
        Self { tag_id, attribute }
    }

    pub fn set_attribute(&mut self, attribute: String) {
        self.attribute = Some(attribute);
    }
}

#[derive(Debug, Clone)]
pub struct StringQLGroup {
    pub prefix: StringQLGroupPrefix,
    pub items: Vec<StringQLTagItem>,
}

// ---------------------------------------------------------
/// Immutable String QL Object \
/// Using `StringQLObjectBuilder` to build object
#[derive(Debug, Clone)]
pub struct StringQLObject {
    excludes: Vec<StringQLTagItem>,

    includes: Vec<StringQLTagItem>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { excludes: Vec::new(), includes: Vec::new(), groups: Vec::new() }
    }

    pub fn get_excludes(&self) -> &Vec<StringQLTagItem> {
        &self.excludes
    }

    pub fn get_includes(&self) -> &Vec<StringQLTagItem>{
        &self.includes
    }

    pub fn get_groups(&self) ->  &Vec<StringQLGroup> {
        &self.groups
    }
    
}

// ---------------------------------------------------------
pub struct StringQLObjectBuilder {
    obj: StringQLObject,
}

impl StringQLObjectBuilder {
    pub fn new() -> Self {
        Self { obj: StringQLObject::new() }
    }

    pub fn add_exclude(&mut self, item: StringQLTagItem) {
        self.obj.excludes.push(item);
    }

    pub fn add_include(&mut self, item: StringQLTagItem) {
        self.obj.includes.push(item);
    }

    pub fn add_group(&mut self, prefix: StringQLGroupPrefix, items: Vec<StringQLTagItem>) {
        self.obj.groups.push(StringQLGroup { prefix, items });
    }

    pub fn build(&self) -> StringQLObject {
        self.obj.to_owned()
    }
}