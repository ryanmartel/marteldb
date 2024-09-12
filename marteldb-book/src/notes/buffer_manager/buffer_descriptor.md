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

### State Field

The Buffer state is a single 32-bit atomic variable where the following are combined  
* 18 bits refcount
* 4 bits usage count
* 10 bits of flags

Combining these values allows to perform some operations without locking the buffer header, by 
modifying them together with a CAS loop.  
```
/* Buffer State bit definitions */
#define BUF_REFCOUNT_ONE 1
#define BUF_REFCOUNT_MASK ((1U << 18) - 1)
#define BUF_USAGECOUNT_MASK 0x003C0000U
#define BUF_USAGECOUNT_ONE (1U << 18)
#define BUF_USAGECOUNT_SHIFT 18
#define BUF_FLAG_MASK 0xFFC00000U

/* Get refcount and usagecount from buffer state */
#define BUF_STATE_GET_REFCOUNT(state) ((state) & BUF_REFCOUNT_MASK)
#define BUF_STATE_GET_USAGECOUNT(state) (((state) & BUF_USAGECOUNT_MASK) >> BUF_USAGECOUNT_SHIFT)

/*
 * Flags for buffer descriptors
 *
 * Note: BM_TAG_VALID essentially means that there is a buffer hashtable
 * entry associated with the buffer's tag.
 */
#define BM_LOCKED               (1U << 22)	/* buffer header is locked */
#define BM_DIRTY                (1U << 23)	/* data needs writing */
#define BM_VALID                (1U << 24)	/* data is valid */
#define BM_TAG_VALID            (1U << 25)	/* tag is assigned */
#define BM_IO_IN_PROGRESS       (1U << 26)	/* read or write in progress */
#define BM_IO_ERROR             (1U << 27)	/* previous I/O failed */
#define BM_JUST_DIRTIED         (1U << 28)	/* dirtied since write started */
#define BM_PIN_COUNT_WAITER     (1U << 29)	/* have waiter for sole pin */
#define BM_CHECKPOINT_NEEDED    (1U << 30)	/* must write for checkpoint */
#define BM_PERMANENT            (1U << 31)	/* permanent buffer (not unlogged,
											 * or init fork) */
```

In addition to the above state bit definitions, the maximum allowed value of usage_count is also 
defined. This creates a tradeoff between the accuracy and speed of the clock-sweep buffer 
management alogrithm. A large value (compared to NBuffers) would approximate LRU semantics. 
But it can take as many as (maximum usage_count + 1) complete cycles of clock sweeps to find a 
free buffer, so in practice the value should not be very high.
```
#define BM_MAX_USAGE_COUNT 5
```

### The Buffer Header lock (BM_LOCKED flag)

The buffer header lock must be held to examine or change tag, state, or wait_backend_pgprocno 
fields. In general, the buffer header lock is a spinlock which is combined with flags, refcount 
and usagecount into a single atomic variable. This allows some single atomic operations such as 
increaseing or decreasing refcount without acquiring and releasing the spinlock.
