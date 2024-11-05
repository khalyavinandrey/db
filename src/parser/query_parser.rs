use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::separated_list1;
use crate::parser::query_parser::parse_tree::{InsertQuery, Queries, Query, SelectQuery};

mod select_parser;
mod insert_parser;
mod parse_tree;

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
