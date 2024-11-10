use crate::parser::{Node, Op};

pub struct Analyzer {}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {}
    }

    pub fn analyze(&self, ast: &Node) -> LogicalPlan {
        LogicalPlan {
            root: self.walk(ast)[0].clone()
        }
    }

    fn walk(&self, node: &Node) -> Vec<LogicalNode> {
        match node.op() {
            Some(Op::Select) => {
                let children = node.children();
                let from_node = children[children.len() - 1].clone();
                let columns_node = children[0].clone();

                let columns = self.build_columns(&columns_node);

                vec![LogicalNode {
                    operator: Operator::Projection(ProjectionInfo {
                        columns
                    }),
                    children: self.walk(&from_node)
                }]
            },
            Some(Op::From) => {
                let children = node.children();
                let tables_node = children[0].clone();

                let tables = self.build_tables(&tables_node);

                let mut nodes = vec![];

                for table in tables {
                    let node = LogicalNode {
                            operator: Operator::Read(ReadInfo {
                                table: Table {
                                    name: table.name
                                }
                            }),
                            children: vec![]
                        };

                    nodes.push(node);
                }

                nodes
            },
            _ => panic!()
        }
    }

    fn build_columns(&self, columns_node: &Node) -> Vec<Column> {
        let mut column_walker = ColumnWalker::new();

        column_walker.walk(&columns_node)
    }

    fn build_tables(&self, tables_node: &Node) -> Vec<Table> {
        let mut table_walker = TableWalker::new();

        table_walker.walk(&tables_node)
    }
}

struct ColumnWalker {
    columns: Vec<Column>
}

impl ColumnWalker {
    fn new() -> Self {
        ColumnWalker {
            columns: vec![]
        }
    }

    fn walk(&mut self, column_node: &Node) -> Vec<Column> {
        match column_node.op() {
            Some(Op::Comma) => {
                let children = column_node.children();

                self.walk(&children[0].clone());
                self.walk(&children[1].clone())
            }
            None => {
                let column_name = column_node.literal().unwrap().get_first_name_as_string();

                self.columns.push(
                    Column {
                        name: column_name
                    }
                );

                self.columns.clone()
            },
            _ => {
                panic!("Unexpected")
            }
        }
    }
}

struct TableWalker {
    tables: Vec<Table>
}

impl TableWalker {
    fn new() -> Self {
        TableWalker {
            tables: vec![]
        }
    }

    fn walk(&mut self, table_node: &Node) -> Vec<Table> {
        match table_node.op() {
            Some(Op::Comma) => {
                let children = table_node.children();

                self.walk(&children[0].clone());
                self.walk(&children[1].clone());

                self.tables.clone()
            },
            None => {
                let table_name = table_node.literal().unwrap().get_first_name_as_string();

                self.tables.push(
                    Table {
                        name: table_name
                    }
                );

                self.tables.clone()
            },
            _ => {
                panic!("Unexpected")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Table {
    name: String
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Projection(ProjectionInfo),
    Filter(FilterInfo),
    Read(ReadInfo),
    Join(JoinInfo),
    Group(GroupInfo),
    Sort(SortInfo),
    Limit(LimitInfo),
    Distinct(DistinctInfo),
}

#[derive(Debug, PartialEq, Clone)]
struct ProjectionInfo {
    columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Clone)]
enum LogicalOperation {
    AndOp(),
    LessThan()
}

#[derive(Debug, PartialEq, Clone)]
struct FilterInfo {

}

#[derive(Debug, PartialEq, Clone)]
struct ReadInfo {
    table: Table
}

#[derive(Debug, PartialEq, Clone)]
struct JoinInfo {

}

#[derive(Debug, PartialEq, Clone)]
struct GroupInfo {}

#[derive(Debug, PartialEq, Clone)]
struct SortInfo {}

#[derive(Debug, PartialEq, Clone)]
struct LimitInfo {}

#[derive(Debug, PartialEq, Clone)]
struct DistinctInfo {}

#[derive(Debug, PartialEq, Clone)]
struct LogicalNode {
    pub operator: Operator,
    pub children: Vec<LogicalNode>,
}

#[derive(Debug, PartialEq)]
pub struct LogicalPlan {
    pub root: LogicalNode,
}

#[derive(Debug, PartialEq, Clone)]
struct Column {
    name: String
}

mod tests {
    use crate::analyzer::{Analyzer, Column, LogicalNode, LogicalPlan, Operator, ProjectionInfo, ReadInfo, Table};
    use crate::parser::lexer::Lexer;
    use crate::parser::Parser;

    fn column(name: &str) -> Column {
        Column { name: name.to_string() }
    }

    fn table(name: &str) -> Table {
        Table { name: name.to_string() }
    }

    fn projection(columns: Vec<Column>) -> Operator {
        Operator::Projection(ProjectionInfo { columns })
    }

    fn read(table: Table) -> Operator {
        Operator::Read(ReadInfo { table })
    }

    fn analyze(input: &str) -> LogicalPlan {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let analyzer = Analyzer::new();

        analyzer.analyze(&parser.parse())
    }

    #[test]
    fn simple_test() {
        let logical_plan = analyze("SELECT col1 FROM table1");

        assert_eq!(
            logical_plan,
            LogicalPlan {
                root: LogicalNode {
                    operator: projection(vec![column("col1")]),
                    children: vec![LogicalNode {
                        operator: read(table("table1")),
                        children: vec![],
                    }]
                }
            }
        )
    }

    #[test]
    fn select_many_columns() {
        let logical_plan = analyze("SELECT col1, col2, col3 FROM table1");

        assert_eq!(
            logical_plan,
            LogicalPlan {
                root: LogicalNode {
                    operator: projection(vec![column("col1"), column("col2"), column("col3")]),
                    children: vec![LogicalNode {
                        operator: read(table("table1")),
                        children: vec![],
                    }]
                }
            }
        )
    }

    #[test]
    fn select_many_tables() {
        let logical_plan = analyze("SELECT col1 FROM table1, table2");

        assert_eq!(
            logical_plan,
            LogicalPlan {
                root: LogicalNode {
                    operator: projection(vec![column("col1")]),
                    children: vec![LogicalNode {
                        operator: read(table("table1")),
                        children: vec![],
                    }, LogicalNode {
                        operator: read(table("table2")),
                        children: vec![],
                    }]
                }
            }
        )
    }
}