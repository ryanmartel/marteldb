# Buffer Table

The buffer table is a hash map, mapping a *buffer_tag* to a *buffer_id*. The number of bucket slots is kept greater than the number of buffer pool slots, but collisions still occur. Collisions are handled by chaining with linked lists.
