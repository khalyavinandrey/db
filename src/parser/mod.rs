mod parser;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, i32, space1},
    multi::separated_list1,
    IResult,
};
use crate::parser::ColumnStatement::ColumnStatementIdentifier;

#[derive(Debug, PartialEq)]
struct Queries {
    queries: Vec<Query>,
}

#[derive(Debug, PartialEq)]
enum Query {
    Select(SelectQuery),
    Insert(InsertQuery),
    Delete(DeleteQuery),
    Update(UpdateQuery),
    CreateTable(CreateTableQuery),
    DropTable(DropTableQuery),
    AlterTable(AlterTableQuery),
}

#[derive(Debug, PartialEq)]
struct SelectQuery {
    select_statement: SelectStatement,
    from_statement: FromStatement,
    where_statement: Option<WhereStatement>,
    order_by_statement: Option<OrderByStatement>,
    group_by_statement: Option<GroupByStatement>,
    having_statement: Option<HavingStatement>,
    limit_statement: Option<LimitStatement>,
}

#[derive(Debug, PartialEq)]
struct InsertQuery {
    table_name: String,
    columns: Vec<String>,
    values: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct DeleteQuery {
    table_name: String,
    where_statement: Option<WhereStatement>,
}

#[derive(Debug, PartialEq)]
struct UpdateQuery {
    table_name: String,
    set_statement: SetStatement,
    where_statement: Option<WhereStatement>,
}

#[derive(Debug, PartialEq)]
struct CreateTableQuery {
    table_name: String,
    columns: Vec<String>,
    constraints: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct DropTableQuery {
    table_name: String,
}

#[derive(Debug, PartialEq)]
struct AlterTableQuery {
    table_name: String,
    action: String,
}

#[derive(Debug, PartialEq)]
struct SelectStatement {
    columns: Vec<ColumnStatement>,
    distinct: bool,
}

/// t1, t2, schema.t3, join t4 on t1.id = t4.id
#[derive(Debug, PartialEq)]
struct FromStatement {
    tables: Vec<TableStatement>,
    joins: Vec<JoinStatement>,
}

#[derive(Debug, PartialEq)]
struct TableStatement {
    table_name: String,
    alias: Option<String>,
}

#[derive(Debug, PartialEq)]
struct JoinStatement {
    table_name: String,
    join_type: String,
    on: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct Condition {
    left: ColumnStatement,
    operator: Operator,
    right: ColumnStatement,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
struct WhereStatement {
    conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct OrderByStatement {
    columns: Vec<ColumnStatement>,
    order: Order,
}

#[derive(Debug, PartialEq)]
enum Order {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
struct GroupByStatement {
    columns: Vec<ColumnStatement>,
}

#[derive(Debug, PartialEq)]
struct HavingStatement {
    conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct LimitStatement {
    limit: i32,
}

#[derive(Debug, PartialEq)]
struct SetStatement {
    // TODO
    columns: Vec<String>,
}

/// *ColumnStatement*
/// might be:
/// col1, tableName.col1, 11, 'lalalal'
#[derive(Debug, PartialEq)]
enum ColumnStatement {
    ColumnStatementIdentifier(ColumnIdentifier),
    ColumnStatementLiteral(Literal),
    ColumnStatementFunction(Function),
}

#[derive(Debug, PartialEq)]
struct ColumnIdentifier {
    table_name: Option<String>,
    column_name: String,
}

#[derive(Debug, PartialEq)]
enum Literal {
    Integer(i32),
    String(String),
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
struct Function {
    name: String,
    args: Vec<ColumnStatement>
}

impl Queries {
    fn parse(input: &str) -> IResult<&str, Queries> {
        let (input, queries) = separated_list1(tag(";"), Query::parse)(input)?;
        Ok((input, Queries { queries }))
    }
}

impl Query {
    fn parse(input: &str) -> IResult<&str, Query> {
        let select_query_attempt = SelectQuery::parse(input);
        if select_query_attempt.is_ok() {
            return select_query_attempt
                .map(|(input, select_query)| (input, Query::Select(select_query)));
        }
        let insert_query_attempt = InsertQuery::parse(input);
        if insert_query_attempt.is_ok() {
            return insert_query_attempt
                .map(|(input, insert_query)| (input, Query::Insert(insert_query)));
        }
        panic!("Query not recognized");
    }
}

impl SelectStatement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, columns) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;

        Ok((
            input,
            SelectStatement {
                columns,
                distinct: false
            },
        ))
    }
}

impl ColumnStatement {
    fn parse(input: &str) -> IResult<&str, ColumnStatement> {
        let res = ColumnIdentifier::parse(input);
        if res.is_ok() {
            return res.map(|(input, column_identifier)| {
                (input, ColumnStatementIdentifier(column_identifier))
            });
        }
        let res = Literal::parse(input);
        if res.is_ok() {
            return res
                .map(|(input, literal)| (input, ColumnStatement::ColumnStatementLiteral(literal)));
        }
        let res = Function::parse(input);
        if res.is_ok() {
            return res.map(|(input, function)| {
                (input, ColumnStatement::ColumnStatementFunction(function))
            });
        }
        panic!("ColumnStatement not recognized");
    }
}

impl ColumnIdentifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, column_name) = alphanumeric1(input)?;

        Ok((
            input,
            ColumnIdentifier {
                table_name: None,
                column_name: column_name.to_string(),
            }),
        )
    }
}

impl Literal {
    fn parse(input: &str) -> IResult<&str, Literal> {
        let res = i32(input);
        if res.is_ok() {
            return res.map(|(input, i_value)| (input, Literal::Integer(i_value)));
        }
        let (input, _) = tag("'")(input)?;
        let (input, s) = alpha1(input)?;
        let (input, _) = tag("'")(input)?;
        Ok((input, Literal::String(s.to_string())))
    }
}

impl Function {
    fn parse(input: &str) -> IResult<&str, Function> {
        let (input, name) = alpha1(input)?;

        let (input, _) = tag("(")(input)?;
        let (input, args) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((
            input,
            Function {
                name: name.to_string(),
                args
            }
        ))
    }
}

impl FromStatement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, table_name) = TableStatement::parse(input)?;
        // let (input, _) = space1(input)?;
        // let (input, joins) = separated_list1(tag(" "), JoinStatement::parse)(input)?;

        Ok((
            input,
            FromStatement {
                tables: vec![table_name],
                joins: vec![]
            }
        ))
    }
}

impl TableStatement {
    fn parse(input: &str) -> IResult<&str, TableStatement> {
        let (input, table_name) = alphanumeric1(input)?;

        Ok((
            input,
            TableStatement {
                table_name: table_name.to_string(),
                alias: None
            }
        ))
    }
}

impl SelectQuery {
    fn parse(input: &str) -> IResult<&str, SelectQuery> {
        let (input, _) = tag("select")(input)?;
        let (input, _) = space1(input)?;
        let (input, select_statement) = SelectStatement::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("from")(input)?;
        let (input, _) = space1(input)?;
        let (input, from_statement) = FromStatement::parse(input)?;
        let (input, _) = tag(";")(input)?;

        Ok((
            input,
            SelectQuery {
                select_statement,
                from_statement,
                where_statement: None,
                order_by_statement: None,
                group_by_statement: None,
                having_statement: None,
                limit_statement: None,
            },
        ))
    }
}

impl InsertQuery {
    fn parse(input: &str) -> IResult<&str, InsertQuery> {
        let (input, _) = tag("insert")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("into")(input)?;
        let (input, _) = space1(input)?;
        let (input, table) = alphanumeric1(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, columns) = parse_columns(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("values")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, values) = parse_columns(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = tag(";")(input)?;

        Ok((
            input,
            InsertQuery {
                table_name: table.to_string(),
                columns: columns.iter().map(|s| s.to_string()).collect(),
                values: values.iter().map(|s| s.to_string()).collect(),
            },
        ))
    }
}

fn parse_columns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alphanumeric1)(input)
}

fn main() {}


#[cfg(test)]
mod test {
    use std::vec;
    use crate::parser::{ColumnIdentifier, ColumnStatement, FromStatement, InsertQuery, Queries, Query, SelectQuery, SelectStatement, TableStatement};

    #[test]
    fn test_select() {
        let (remainder, queries) = Queries::parse("select col1, col2 from table1;").unwrap();

        assert_eq!(remainder, "");
        assert_eq!(
            queries,
            Queries {
                queries: vec![Query::Select(SelectQuery {
                    select_statement: SelectStatement {
                        columns: vec![
                            ColumnStatement::ColumnStatementIdentifier(ColumnIdentifier {
                                table_name: None,
                                column_name: "col1".to_string()
                            }),
                            ColumnStatement::ColumnStatementIdentifier(ColumnIdentifier {
                                table_name: None,
                                column_name: "col2".to_string()
                            }),
                        ],
                        distinct: false
                    },
                    from_statement: FromStatement {
                        tables: vec![TableStatement {
                            table_name: "table1".to_string(),
                            alias: None
                        }],
                        joins: vec![]
                    },
                    where_statement: None,
                    order_by_statement: None,
                    group_by_statement: None,
                    having_statement: None,
                    limit_statement: None,
                })]
            }
        );
    }

    #[test]
    fn test_insert() {
        let (remainder, insert_query) =
            Queries::parse("insert into table1 (col1, col2) values (1, valStr);").unwrap();

        assert_eq!(remainder, "");
        assert_eq!(
            insert_query,
            Queries {
                queries: vec![Query::Insert(InsertQuery {
                    table_name: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()],
                    values: vec!["1".to_string(), "valStr".to_string()]
                })]
            }
        );
    }
}
