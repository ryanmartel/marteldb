# Write Ahead Logging WAL

## General Description
Postgres writes all modifications as history data into a persistent storage to prepare for failures. The history data are known as **XLOG record(s)** or **WAL data**.  

XLOG records are written to an in-memory WAL buffer by change operations such as insertion, deletion, or commit action. They are immediately written into a **WAL segment file** when a transaction commits or aborts. The **LSN (Log Sequence Number)** of an XLOG record represents the location where its record is written on the transaction log. The LSN of a record is used as the unique id of the XLOG record.  

The location to write the XLOG record at the moment when the latest **checkpoint** is started is known as the **REDO point**.  

## Transaction log and WAL segment files

Postgres writes XLOG records into a virtual file that is 8 bytes of address space long. A file with this address space would yeild a rediculous sized file. Therefore, postgres divides this address space amont 16 MB segment files. 

The segment filename is a hexadecimal 24-digit number and is named by:  
'''
WAL segment file name = timelineId + (uint32)
'''
