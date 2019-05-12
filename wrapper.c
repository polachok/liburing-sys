#include <string.h>
#include "wrapper.h"

void _io_uring_prep_rw(int op, struct io_uring_sqe *sqe, int fd,
				    void *addr, unsigned len, off_t offset) {
	io_uring_prep_rw(op, sqe, fd, addr, len, offset);
}

void _io_uring_cqe_seen(struct io_uring *ring, struct io_uring_cqe *cqe) {
	io_uring_cqe_seen(ring, cqe);
}

void _io_uring_prep_readv(struct io_uring_sqe *sqe, int fd,
				       struct iovec *iovecs, unsigned nr_vecs,
				       off_t offset) {
	io_uring_prep_readv(sqe, fd, iovecs, nr_vecs, offset);
}

void _io_uring_prep_read_fixed(struct io_uring_sqe *sqe, int fd,
					    void *buf, unsigned nbytes,
					    off_t offset) {

	io_uring_prep_read_fixed(sqe, fd, buf, nbytes, offset);
}

void _io_uring_prep_writev(struct io_uring_sqe *sqe, int fd,
				        struct iovec *iovecs, unsigned nr_vecs,
					off_t offset)
{
	io_uring_prep_writev(sqe, fd, iovecs, nr_vecs, offset);
}

void _io_uring_prep_write_fixed(struct io_uring_sqe *sqe, int fd,
					     void *buf, unsigned nbytes,
					     off_t offset)
{
	io_uring_prep_write_fixed(sqe, fd, buf, nbytes, offset);
}


void _io_uring_prep_fsync(struct io_uring_sqe *sqe, int fd,
				       unsigned fsync_flags) {

	io_uring_prep_fsync(sqe, fd, fsync_flags);
}

void _io_uring_prep_nop(struct io_uring_sqe *sqe) {
	io_uring_prep_nop(sqe);
}
