use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum NodeType {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue,
    ContractDefinition,
    DoWhileStatement,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    ErrorDefinition,
    EventDefinition,
    ExpressionStatement,
    ForStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    Mapping,
    MemberAccess,
    ModifierDefinition,
    ModifierInvocation,
    NewExpression,
    OverrideSpecifier,
    ParameterList,
    PlaceholderStatement,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    Throw,
    TryCatchClause,
    TryStatement,
    TupleExpression,
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    YulAssignment,
    YulBlock,
    YulCase,
    YulExpressionStatement,
    YulFunctionCall,
    YulIdentifier,
    YulLiteral,
    YulSwitch,
    YulTypedName,
    YulVariableDeclaration,
    Unknown,
}

// Common
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Payable,
    NonPayable,
    Pure,
    View,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Internal,
    Public,
    External,
}

// FunctionDeclaration
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Receive,
    Fallback,
    FreeFunction,
    Function,
}

// Block
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: u32,
    pub src: String,
    pub statements: Vec<Statement>,
}

// ExpressionStatement
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: u32,
    pub src: String,
    pub expression: Expression,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: u32,
    pub src: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: u32,
    pub src: String,
    pub name: String,
}

// Expression
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall(FunctionCall),
    FunctionCallOptions,
    Identifier(Identifier),
    IndexAccess,
    IndexRangeAccess,
    Literal,
    MemberAccess,
    NewExpression,
    TupleExpression,
    UnaryOperation,
}

// Statement
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break,
    Continue,
    DoWhileStatement,
    EmitStatement,
    ExpressionStatement(ExpressionStatement),
    ForStatement,
    IfStatement,
    InlineAssembly,
    PlaceholderStatement,
    Return,
    RevertStatement,
    TryStatement,
    UncheckedBlock,
    VariableDeclarationStatement,
    WhileStatement,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Node {
    ArrayTypeName {},
    Assignment {},
    BinaryOperation {},
    Block(Block),
    Break {},
    Conditional {},
    Continue {},
    ContractDefinition {
        id: u32,
        nodes: Vec<Node>,
        name: String,
    },
    DoWhileStatement {},
    ElementaryTypeName {},
    ElementaryTypeNameExpression {},
    EmitStatement {},
    EnumDefinition {},
    EnumValue {},
    ErrorDefinition {},
    EventDefinition {},
    ExpressionStatement {},
    ForStatement {},
    FunctionCall {},
    FunctionCallOptions {},
    #[serde(rename_all = "camelCase")]
    FunctionDefinition {
        id: u32,
        nodes: Vec<Node>,
        kind: FunctionKind,
        name: String,
        visibility: Visibility,
        state_mutability: StateMutability,
        body: Block,
    },
    FunctionTypeName {},
    Identifier {},
    IdentifierPath {},
    IfStatement {},
    ImportDirective {},
    IndexAccess {},
    IndexRangeAccess {},
    InheritanceSpecifier {},
    InlineAssembly {},
    Literal {},
    Mapping {},
    MemberAccess {},
    ModifierDefinition {},
    ModifierInvocation {},
    NewExpression {},
    OverrideSpecifier {},
    ParameterList {},
    PlaceholderStatement {},
    PragmaDirective {},
    Return {},
    RevertStatement {},
    SourceUnit { id: u32, nodes: Vec<Node> },
    StructDefinition {},
    StructuredDocumentation {},
    Throw {},
    TryCatchClause {},
    TryStatement {},
    TupleExpression {},
    UnaryOperation {},
    UncheckedBlock {},
    UserDefinedTypeName {},
    UserDefinedValueTypeDefinition {},
    UsingForDirective {},
    VariableDeclaration {
        id: u32,
        nodes: Vec<Node>,
        name: String,
        visibility: Visibility,
    },
    VariableDeclarationStatement {},
    WhileStatement {},
    YulAssignment {},
    YulBlock {},
    YulCase {},
    YulExpressionStatement {},
    YulFunctionCall {},
    YulIdentifier {},
    YulLiteral {},
    YulSwitch {},
    YulTypedName {},
    YulVariableDeclaration {},
    #[serde(untagged)]
    Unknown { id: u32, nodes: Vec<Node> },
}

impl Node {
    pub fn get_type(&self) -> NodeType {
        match self {
            Node::ArrayTypeName { .. } => NodeType::ArrayTypeName,
            Node::Assignment { .. } => NodeType::Assignment,
            Node::BinaryOperation { .. } => NodeType::BinaryOperation,
            Node::Block { .. } => NodeType::Block,
            Node::Break { .. } => NodeType::Break,
            Node::Conditional { .. } => NodeType::Conditional,
            Node::Continue { .. } => NodeType::Continue,
            Node::ContractDefinition { .. } => NodeType::ContractDefinition,
            Node::DoWhileStatement { .. } => NodeType::DoWhileStatement,
            Node::ElementaryTypeName { .. } => NodeType::ElementaryTypeName,
            Node::ElementaryTypeNameExpression { .. } => NodeType::ElementaryTypeNameExpression,
            Node::EmitStatement { .. } => NodeType::EmitStatement,
            Node::EnumDefinition { .. } => NodeType::EnumDefinition,
            Node::EnumValue { .. } => NodeType::EnumValue,
            Node::ErrorDefinition { .. } => NodeType::ErrorDefinition,
            Node::EventDefinition { .. } => NodeType::EventDefinition,
            Node::ExpressionStatement { .. } => NodeType::ExpressionStatement,
            Node::ForStatement { .. } => NodeType::ForStatement,
            Node::FunctionCall { .. } => NodeType::FunctionCall,
            Node::FunctionCallOptions { .. } => NodeType::FunctionCallOptions,
            Node::FunctionDefinition { .. } => NodeType::FunctionDefinition,
            Node::FunctionTypeName { .. } => NodeType::FunctionTypeName,
            Node::Identifier { .. } => NodeType::Identifier,
            Node::IdentifierPath { .. } => NodeType::IdentifierPath,
            Node::IfStatement { .. } => NodeType::IfStatement,
            Node::ImportDirective { .. } => NodeType::ImportDirective,
            Node::IndexAccess { .. } => NodeType::IndexAccess,
            Node::IndexRangeAccess { .. } => NodeType::IndexRangeAccess,
            Node::InheritanceSpecifier { .. } => NodeType::InheritanceSpecifier,
            Node::InlineAssembly { .. } => NodeType::InlineAssembly,
            Node::Literal { .. } => NodeType::Literal,
            Node::Mapping { .. } => NodeType::Mapping,
            Node::MemberAccess { .. } => NodeType::MemberAccess,
            Node::ModifierDefinition { .. } => NodeType::ModifierDefinition,
            Node::ModifierInvocation { .. } => NodeType::ModifierInvocation,
            Node::NewExpression { .. } => NodeType::NewExpression,
            Node::OverrideSpecifier { .. } => NodeType::OverrideSpecifier,
            Node::ParameterList { .. } => NodeType::ParameterList,
            Node::PlaceholderStatement { .. } => NodeType::PlaceholderStatement,
            Node::PragmaDirective { .. } => NodeType::PragmaDirective,
            Node::Return { .. } => NodeType::Return,
            Node::RevertStatement { .. } => NodeType::RevertStatement,
            Node::SourceUnit { .. } => NodeType::SourceUnit,
            Node::StructDefinition { .. } => NodeType::StructDefinition,
            Node::StructuredDocumentation { .. } => NodeType::StructuredDocumentation,
            Node::Throw { .. } => NodeType::Throw,
            Node::TryCatchClause { .. } => NodeType::TryCatchClause,
            Node::TryStatement { .. } => NodeType::TryStatement,
            Node::TupleExpression { .. } => NodeType::TupleExpression,
            Node::UnaryOperation { .. } => NodeType::UnaryOperation,
            Node::UncheckedBlock { .. } => NodeType::UncheckedBlock,
            Node::UserDefinedTypeName { .. } => NodeType::UserDefinedTypeName,
            Node::UserDefinedValueTypeDefinition { .. } => NodeType::UserDefinedValueTypeDefinition,
            Node::UsingForDirective { .. } => NodeType::UsingForDirective,
            Node::VariableDeclaration { .. } => NodeType::VariableDeclaration,
            Node::VariableDeclarationStatement { .. } => NodeType::VariableDeclarationStatement,
            Node::WhileStatement { .. } => NodeType::WhileStatement,
            Node::YulAssignment { .. } => NodeType::YulAssignment,
            Node::YulBlock { .. } => NodeType::YulBlock,
            Node::YulCase { .. } => NodeType::YulCase,
            Node::YulExpressionStatement { .. } => NodeType::YulExpressionStatement,
            Node::YulFunctionCall { .. } => NodeType::YulFunctionCall,
            Node::YulIdentifier { .. } => NodeType::YulIdentifier,
            Node::YulLiteral { .. } => NodeType::YulLiteral,
            Node::YulSwitch { .. } => NodeType::YulSwitch,
            Node::YulTypedName { .. } => NodeType::YulTypedName,
            Node::YulVariableDeclaration { .. } => NodeType::YulVariableDeclaration,
            Node::Unknown { .. } => NodeType::Unknown,
        }
    }

    pub fn get_nodes(&self) -> Option<&Vec<Node>> {
        match self {
            Node::SourceUnit { nodes, .. } => Some(&nodes),
            Node::ContractDefinition { nodes, .. } => Some(&nodes),
            Node::FunctionDefinition { nodes, .. } => Some(&nodes),
            Node::Unknown { nodes, .. } => Some(&nodes),
            _ => None,
        }
    }
}
