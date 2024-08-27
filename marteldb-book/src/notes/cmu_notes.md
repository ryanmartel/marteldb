# CMU database

## Storage/Execution Flow EX.
1. Execution Engine requests to get page from buffer pool
2. If buffer pool does not have the page in memory, it must be fetched from disk
3. Bring page directory into memory if it is not already present
4. Look up the page location using the page directory and fetch it from disk
5. Give a pointer to the buffer pool page back to the execution engine
6. The execution engine can then interpret (deserialize) and operate on that page's data
7. The page, if now dirty, can be written back out to its location on disk
