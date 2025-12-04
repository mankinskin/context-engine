# Desired Search Algorithm

## Goal
Find the largest match at the beginning of an input vector of tokens given a search graph.

## Initialization

1. **Start State**:
   - Advance query to first token in pattern
   - Examine first set of parents in search graph (larger contexts with potential matches)
   - Initialize best match with first token as matched
   - Return error if query is empty

2. **Parent State Tracking** (for each parent):
   - **Path**: Bottom-up edges to parent from "last match"
     - Includes root entry and root parent in graph
   - **Atom Offset**: Atom width of all tokens matched before exploring this parent
     - Equals matched query cursor position
     - Represents entry point of parent state
   - **Query Path**: Search path in query with query pattern as root
     - Includes root entry
   - **Query Cursor Position**: Atom width of "explored" tokens
     - For matched cursors: atom width of matched tokens including end leaf
     - For candidate cursors: atom width of last match including candidate end leaf token

## Search Strategy

### Bottom-Up Exploration
- Explore all parents bottom-up in **ascending width order**
- Inconclusive root candidates appended to search queue in BFS manner with extra ordering
- Priority: smaller tokens processed first

### Comparison Process

1. **Create Candidate Root Cursor**:
   - From parent state
   - Next token after parent state root entry as end leaf
   - Start path computed from last match root

2. **Compare End Leafs**:
   - Compare end leafs of candidate query and index paths
   - **Inconclusive**: If end leafs are not same width
     - Append prefix states to search queue
     - Continue searching in BFS manner

3. **Match Found**:
   - **Reinitiialize search queue** - all larger matches must be parents of this root
   - Or final match ends in this root (due to substring-graph structure)
   - **Best match tracking**: Keep track of best match when finding new matching root
   - **Clear search queue** when best match updated

### Finding End in Matched Root

After finding first match in a root:

1. **Advance Query Cursor**: To next candidate token from query
2. **Advance Index Path**: Into remaining root
3. **Inconclusive Case**: 
   - If reached end of root
   - Continue exploring root's parents with candidate query
4. **Continue Comparison**:
   - Compare both candidate tokens
   - Until either:
     - Mismatch found within matched root, OR
     - Query ends

## Outcomes

- **Match**: Comparison succeeded
- **Mismatch**: Comparison failed
- **Inconclusive**: Need decomposition/parent exploration

## Trace Cache Management

### When Returning Match Result

1. **Commit All Atom Position Traces**:
   - Ensure all traces for result added to trace cache

2. **Trace End Path**:
   - Commit final end position to trace cache

3. **Incremental Start Path Tracing**:
   - Start paths committed to trace cache incrementally
   - While finding larger matching roots
   - Must be contained in final root

## Key Invariants

1. **Best Match**: Always track best match at each new matching root
2. **Queue Management**: Clear queue when best match updated
3. **Ascending Width**: Process parents in ascending width order
4. **BFS with Ordering**: Maintain breadth-first search with extra ordering for priority
5. **Substring-Graph Structure**: Leverage inherent structure where larger matches are parents
