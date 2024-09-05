# Buffer Manager

### Important data structures
**BufferTag**: A unique BufferTag is assigned to each data page. The buffer tag contains  
1. *dbOid* - The OID of the database to which the relation containig the target page belongs.
2. *relNumber* - The number of the relation file that contains the target page.
3. *blockNum* - The block number of the target page in the relation.
4. *forkNum* - The fork number of the relation that the page belongs to. The fork numbers of tables, freespace maps, and visibility maps are defined in 0, 1, 2, respectfully.  
  
**Buffer Pool**: An array that stores data file pages. Each slot in the array is referred to as a *buffer_id*

**Buffer Descriptors**: An array of buffer descriptors. Each has a one-to-one correspondance to a buffer pool slot and holds the metadata of the stored page.


**Buffer Table**: Hash table that stores the relations between the *buffer_tag* of stored pages and *buffer_id* of the descriptors that hold the stored pages' metadata.

