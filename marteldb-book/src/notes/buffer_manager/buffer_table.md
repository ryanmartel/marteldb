# Buffer Table

The buffer table is a hash map, mapping a *buffer_tag* to a *buffer_id*. The number of bucket slots is kept greater than the number of buffer pool slots, but collisions still occur. Collisions are handled by chaining with linked lists.

## Locks

**BufMappingLock** protects the integrity of the entire buffer table. It can be taken in either shared or exclusive mode.  

A backend process holds this lock in the following modes:
Shared - when searching an entry in the buffer table
Exclusive - when inserting or deleting entries

The BufMappingLock in postgres is split into partitions to reduce lock contention. (Default 128 partitions). Each partition guards only a portion of the hash bucket slots. The buffer table may use other locks, such as a spin lock to delete entries.
