#[derive(Debug, PartialEq)]
pub struct Queries {
    pub queries: Vec<Query>,
}

#[derive(Debug, PartialEq)]
pub enum Query {
    Select(SelectQuery),
    Insert(InsertQuery),
    Delete(DeleteQuery),
    Update(UpdateQuery),
    CreateTable(CreateTableQuery),
    DropTable(DropTableQuery),
    AlterTable(AlterTableQuery),
}

#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    pub select_statement: SelectStatement,
    pub from_statement: FromStatement,
    pub  where_statement: Option<WhereStatement>,
    pub order_by_statement: Option<OrderByStatement>,
    pub group_by_statement: Option<GroupByStatement>,
    pub having_statement: Option<HavingStatement>,
    pub limit_statement: Option<LimitStatement>,
}

#[derive(Debug, PartialEq)]
pub struct InsertQuery {
    pub  table_name: String,
    pub columns: Vec<String>,
    pub  values: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct DeleteQuery {
    pub table_name: String,
    pub  where_statement: Option<WhereStatement>,
}

#[derive(Debug, PartialEq)]
pub struct UpdateQuery {
    pub table_name: String,
    pub set_statement: SetStatement,
    pub where_statement: Option<WhereStatement>,
}

#[derive(Debug, PartialEq)]
pub struct CreateTableQuery {
    pub  table_name: String,
    pub  columns: Vec<String>,
    pub  constraints: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct DropTableQuery {
    pub table_name: String,
}

#[derive(Debug, PartialEq)]
pub struct AlterTableQuery {
    pub table_name: String,
    pub action: String,
}

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub columns: Vec<ColumnStatement>,
    pub  distinct: bool,
}

/// t1, t2, schema.t3, join t4 on t1.id = t4.id
#[derive(Debug, PartialEq)]
pub struct FromStatement {
    pub  tables: Vec<TableStatement>,
    pub joins: Vec<JoinStatement>,
}

#[derive(Debug, PartialEq)]
pub struct TableStatement {
    pub table_name: String,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct JoinStatement {
    pub table_name: String,
    pub join_type: String,
    pub  on: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub  left: ColumnStatement,
    pub operator: Operator,
    pub right: ColumnStatement,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
pub struct WhereStatement {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
pub struct OrderByStatement {
    pub columns: Vec<ColumnStatement>,
    pub order: Order,
}

#[derive(Debug, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
pub struct GroupByStatement {
    pub columns: Vec<ColumnStatement>,
}

#[derive(Debug, PartialEq)]
pub struct HavingStatement {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
pub struct LimitStatement {
    pub limit: i32,
}

#[derive(Debug, PartialEq)]
pub struct SetStatement {
    // TODO
    pub columns: Vec<String>,
}

/// *ColumnStatement*
/// might be:
/// col1, tableName.col1, 11, 'lalalal'
#[derive(Debug, PartialEq)]
pub enum ColumnStatement {
    ColumnStatementIdentifier(ColumnIdentifier),
    ColumnStatementLiteral(Literal),
    ColumnStatementFunction(Function),
}

#[derive(Debug, PartialEq)]
pub struct ColumnIdentifier {
    pub table_name: Option<String>,
    pub column_name: String,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i32),
    String(String),
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<ColumnStatement>
}
