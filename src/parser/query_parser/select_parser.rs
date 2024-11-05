use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, i32, space1};
use nom::IResult;
use nom::multi::separated_list1;
use crate::parser::query_parser::parse_tree::{ColumnIdentifier, ColumnStatement, FromStatement, Function, Literal, SelectQuery, SelectStatement, TableStatement};
use crate::parser::query_parser::parse_tree::ColumnStatement::ColumnStatementIdentifier;

impl SelectQuery {
    pub fn parse(input: &str) -> IResult<&str, SelectQuery> {
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

impl SelectStatement {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, columns) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;

        Ok((
            input,
            SelectStatement {
                columns,
                distinct: false,
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
                args,
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
                joins: vec![],
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
                alias: None,
            }
        ))
    }
}

#[test]
fn test_select() {
    let (remainder, queries) = SelectQuery::parse("select col1, col2 from table1;").unwrap();

    assert_eq!(remainder, "");
    assert_eq!(
        queries,
        SelectQuery {
            select_statement: SelectStatement {
                columns: vec![
                    ColumnStatementIdentifier(ColumnIdentifier {
                        table_name: None,
                        column_name: "col1".to_string()
                    }),
                    ColumnStatementIdentifier(ColumnIdentifier {
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
        })
}