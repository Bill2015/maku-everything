use super::AttributeValue;

#[derive(Debug, Clone)]
pub enum StringQLPrefix {
    Include,
    Exclude,
    Inherit,
}

// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub struct StringQLItem {
    prefix: StringQLPrefix,

    is_system: bool,

    value: String,

    attribute: Option<AttributeValue>,
}
impl StringQLItem {
    pub fn new( prefix: StringQLPrefix, value: String, attribute: Option<AttributeValue>, is_system: bool) -> Self {
        Self { prefix, value, attribute, is_system }
    }

    pub fn get_attribute(&self) -> &Option<AttributeValue> {
        &self.attribute
    }

    pub fn is_system(&self) -> &bool {
        &self.is_system
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_prefix(&self) -> &StringQLPrefix {
        &self.prefix
    }
}

#[derive(Debug, Clone)]
pub struct StringQLGroup {
    pub prefix: StringQLPrefix,

    pub items: Vec<StringQLItem>,
}

// ---------------------------------------------------------
/// Immutable String QL Object \
/// Using `StringQLObjectBuilder` to build object
#[derive(Debug, Clone)]
pub struct StringQLObject {
    belong_category: Option<String>,

    item: Vec<StringQLItem>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { item: Vec::new(), groups: Vec::new(), belong_category: None }
    }

    pub fn get_items(&self) -> &Vec<StringQLItem> {
        &self.item
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

    pub fn set_belong_category(&mut self, belong_category: String) {
        self.obj.belong_category = Some(belong_category);
    }

    pub fn add_group(&mut self, prefix: StringQLPrefix, items: Vec<StringQLItem>) {
        self.obj.groups.push(StringQLGroup { prefix, items });
    }

    pub fn add_item(&mut self, item: StringQLItem) {
        self.obj.item.push(item);
    }

    pub fn build(&self) -> StringQLObject {
        self.obj.to_owned()
    }
}