# Buffer Descriptor

Array of buffer descriptors. Each descriptor has a one-to-one correspondance to a buffer pool slot and holds the metadata of the stored page.

## Structure

At version 9.6 and later, various locks were combined and atomic CPU opertations used.  

* Tag - holds the buffer_tag of the stored page
* buf_id - identifies the desciptor
* content_lock - light-weight lock used to control access to the associated stored page
* freeNext - pointer to the next descriptor to generate a *freelist*.
* states (32-bit atomic) - can hold several states and variables of the associated stored page, such as refcount and usage_count.

```
typedef stuct BufferDesc
{
    BuferTag    tag;            /* ID of page contained in buffer */
    int         buf_id;         /* buffer's index number (from 0) */

    /* state of the tag, containing flags, refcount and usagecount */
    pg_atomic_uint32 state;

    int         wait_backend_pgprocno; /* backend of pin-count waiter */
    int         freeNext;       /* link in freeList chain */
    LWLock      content_lock;   /* to lock access to buffer contents */
} BufferDesc;
```

It is assumed that nobody changes the state field while buffer header lock is held.
