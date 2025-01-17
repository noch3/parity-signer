use parity_scale_codec_derive::{Decode, Encode};

/// Struct to store type name and description
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct TypeEntry {
    pub name: String,
    pub description: Description,
}

/// Type description
#[derive(Decode, Encode, PartialEq, Clone)]
pub enum Description {
    Type(String),
    Enum(Vec<EnumVariant>),
    Struct(Vec<StructField>)
}

/// Enum variants
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct EnumVariant {
    pub variant_name: String,
    pub variant_type: EnumVariantType,
}

/// Types of enum variants
#[derive(Decode, Encode, PartialEq, Clone)]
pub enum EnumVariantType {
    None,
    Type(String),
    Struct(Vec<StructField>),
}

/// Struct fields (field name is optional)
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct StructField {
    pub field_name: Option<String>,
    pub field_type: String,
}

/// Struct to store types updates in history log
pub struct TypesUpdate {
    pub types_hash: String,
    pub verifier_line: String,
}

impl TypesUpdate {
    pub fn show(&self) -> String {
        format!("\"types_hash\":\"{}\",\"verifier\":{}", &self.types_hash, &self.verifier_line)
    }
}
