# Concurrency Control

## Table-level locks
This is the list of commonly used table-level locks in Postgres. Transactions to not ever 
conflict with themselves when locking. Non-conflicting locks may be held by multiple 
transactions  
* **Access Share** Conflicts with **Access Exclusive** lock only. The **SELECT** command acquires 
this lock on referenced tables. In general, queries that read a table without modifying it 
aquire this lock.  
