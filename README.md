





# High level Design

## Network

### Considerations
- Security is completely out of scope
- Custom protocol over TCP

## SQL

### Parser
- Takes string
- Checks for syntax errors
- Returns a parse tree

### Analyzer
- Takes parse tree
- Checks for semantic errors
- Returns a query plan

### Optimizer
- Takes a query plan
- Optimizes the query plan
- Returns an optimized query plan (physical plan)

## Execution
- Takes a physical plan
- Executes the query
- Returns the result (format?)

## Storage
- Declares methods:
    - 'get_by_key(key: bytes) -> bytes'
    - 'put(key: bytes, value: bytes) -> None'
    - 'delete(key: bytes) -> None'
    - 'scan(start_key: bytes, end_key: bytes) -> Iterator[Tuple[bytes, bytes]]'

## Transactions
- Guarantee ACID -> WAL

## Configuration management
- config file
