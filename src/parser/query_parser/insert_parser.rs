use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space1};
use nom::IResult;
use nom::multi::separated_list1;
use crate::parser::query_parser::parse_tree::{InsertQuery, Queries, Query};

impl InsertQuery {
    pub fn parse(input: &str) -> IResult<&str, InsertQuery> {
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

#[test]
fn test_insert() {
    let (remainder, insert_query) =
        InsertQuery::parse("insert into table1 (col1, col2) values (1, valStr);").unwrap();

    assert_eq!(remainder, "");
    assert_eq!(
        insert_query,
        InsertQuery {
            table_name: "table1".to_string(),
            columns: vec!["col1".to_string(), "col2".to_string()],
            values: vec!["1".to_string(), "valStr".to_string()]
        }
    )
}