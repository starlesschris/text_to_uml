use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub enum DiagramType{
    ClassDiagram
}




#[derive(Debug,PartialEq)]
pub enum ClassType{
    SimpleClass,
    AbstractClass,
    ActiveClass,
    VarBorderClass,
    DashedBorderClass,
    None
}

#[derive(Debug,PartialEq)]
pub enum RelationType{
    Association,
    Inheritance,
    Implementation,
    Dependency,
    Aggregation,
    Composition,
    None
}

#[derive(Debug,PartialEq)]
pub enum RelationArrow{
    Arrow,
    TriangleEmpty,
    DiamondEmpty,
    DiamondFilled,
    None
}

#[derive(Debug,PartialEq)]
pub enum BorderType{
    Solid,
    Dashed,
    None
}

#[derive(Debug,PartialEq)]
pub enum TextDecoration{
    Bold,
    Italic,
    BoldItalic,
    Underlined,
    HorizontalLine,
    None
}

#[derive(Debug,PartialEq)]
pub enum Visibility{
    Public,
    Package,
    Protected,
    Private,
    None
}

#[derive(Debug)]
pub struct Class{
    pub class_type: ClassType,
    pub class_name: String,
    pub class_stereotype: String,
    pub border_width: i32,
    pub content_lines: Vec<String>,
    pub content_decor: Vec<TextDecoration>
}

#[derive(Debug)]
pub struct Relation{
    pub border_type: BorderType,
    pub arrow_type: RelationArrow,
    pub from_class: String,
    pub from_class_card: String,
    pub to_class: String,
    pub to_class_card: String
}

#[derive(Debug)]
pub struct ClassDiagram{
    pub classes: Vec<Class>,
    pub relations: Vec<Relation>
}