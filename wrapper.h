#include "liburing/src/liburing.h"

extern void _io_uring_prep_rw(int op, struct io_uring_sqe *sqe, int fd,
				    void *addr, unsigned len, off_t offset);
extern void _io_uring_cqe_seen(struct io_uring *ring, struct io_uring_cqe *cqe);
extern void _io_uring_prep_readv(struct io_uring_sqe *sqe, int fd,
				       struct iovec *iovecs, unsigned nr_vecs,
				       off_t offset);
extern void _io_uring_prep_read_fixed(struct io_uring_sqe *sqe, int fd,
					    void *buf, unsigned nbytes,
					    off_t offset);
extern void _io_uring_prep_writev(struct io_uring_sqe *sqe, int fd,
				        struct iovec *iovecs, unsigned nr_vecs,
					off_t offset);
extern void _io_uring_prep_write_fixed(struct io_uring_sqe *sqe, int fd,
					     void *buf, unsigned nbytes,
					     off_t offset);
extern void _io_uring_prep_fsync(struct io_uring_sqe *sqe, int fd,
				       unsigned fsync_flags);
extern void _io_uring_prep_nop(struct io_uring_sqe *sqe);
