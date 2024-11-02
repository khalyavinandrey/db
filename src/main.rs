use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, i32, space1},
    multi::separated_list1,
    IResult,
};

fn main() {}

#[derive(Debug, PartialEq)]
struct Query {
    select: Option<SelectQuery>,
    insert: Option<InsertQuery>,
}

#[derive(Debug, PartialEq)]
struct SelectQuery {
    table: String,
    columns: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct InsertQuery {
    table: String,
    columns: Vec<String>,
    values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
struct Value {
    i_value: Option<i32>,
    s_value: Option<String>,
}

fn parse_select(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("select")(input)?;
    let (input, _) = space1(input)?;
    let (input, columns) = parse_columns(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Query {
            select: Some(SelectQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect(),
            }),
            insert: None,
        },
    ))
}

fn parse_multi_values(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list1(tag(", "), parse_value)(input)
}

fn parse_columns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alphanumeric1)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let res = i32(input);

    if res.is_ok() {
        return res.map(|(input, i_value)| {
            (
                input,
                Value {
                    i_value: Some(i_value),
                    s_value: None,
                },
            )
        });
    }

    let res = alpha1(input);

    if res.is_ok() {
        return res.map(|(input, s_value)| {
            (
                input,
                Value {
                    i_value: None,
                    s_value: Some(s_value.to_string()),
                },
            )
        });
    }

    let (input, _) = tag("'")(input)?;
    let (input, s) = alpha1(input)?;
    let (input, _) = tag("'")(input)?;

    Ok((
        input,
        Value {
            i_value: None,
            s_value: Some(s.to_string()),
        },
    ))
}

fn parse_insert(input: &str) -> IResult<&str, Query> {
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
    let (input, values) = parse_multi_values(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Query {
            select: None,
            insert: Some(InsertQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect(),
                values: values,
            }),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Query> {
    alt((parse_select, parse_insert))(input)
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{parse, InsertQuery, Query, SelectQuery, Value};

    #[test]
    fn test_select() {
        let (remainder, query) = parse("select col1, col2 from table1;").expect("error");

        assert_eq!(remainder, "");
        assert_eq!(
            query,
            Query {
                select: Some(SelectQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()]
                }),
                insert: None,
            }
        )
    }

    #[test]
    fn test_insert() {
        let (remainder, query) =
            parse("insert into table1 (col1, col2) values (1, 'valStr');").expect("err");

        assert_eq!(remainder, "");
        assert_eq!(
            query,
            Query {
                select: None,
                insert: Some(InsertQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()],
                    values: vec![
                        Value {
                            i_value: Some(1),
                            s_value: None,
                        },
                        Value {
                            i_value: None,
                            s_value: Some("valStr".to_string()),
                        }
                    ]
                }),
            }
        )
    }
}
