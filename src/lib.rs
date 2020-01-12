#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

use libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const IORING_OP_NOP: libc::c_int = 0;
pub const IORING_OP_READV: libc::c_int = 1;
pub const IORING_OP_WRITEV: libc::c_int = 2;
pub const IORING_OP_FSYNC: libc::c_int = 3;
pub const IORING_OP_READ_FIXED: libc::c_int = 4;
pub const IORING_OP_WRITE_FIXED: libc::c_int = 5;

pub unsafe fn io_uring_prep_readv(
    sqe: *mut io_uring_sqe,
    fd: libc::c_int,
    iovecs: *mut libc::iovec,
    nr_vecs: libc::c_uint,
    offset: libc::off_t,
) {
    _io_uring_prep_readv(sqe, fd, core::mem::transmute(iovecs), nr_vecs, offset)
}

pub unsafe fn io_uring_prep_read_fixed(
    sqe: *mut io_uring_sqe,
    fd: libc::c_int,
    buf: *mut libc::c_void,
    nbytes: libc::c_uint,
    offset: libc::off_t,
) {
    _io_uring_prep_read_fixed(sqe, fd, buf, nbytes, offset)
}

pub unsafe fn io_uring_prep_writev(
    sqe: *mut io_uring_sqe,
    fd: libc::c_int,
    iovecs: *mut libc::iovec,
    nr_vecs: libc::c_uint,
    offset: libc::off_t,
) {
    _io_uring_prep_writev(sqe, fd, core::mem::transmute(iovecs), nr_vecs, offset)
}

pub unsafe fn io_uring_prep_write_fixed(
    sqe: *mut io_uring_sqe,
    fd: libc::c_int,
    buf: *mut libc::c_void,
    nbytes: libc::c_uint,
    offset: libc::off_t,
) {
    _io_uring_prep_write_fixed(sqe, fd, buf, nbytes, offset)
}

pub unsafe fn io_uring_prep_fsync(
    sqe: *mut io_uring_sqe,
    fd: libc::c_int,
    fsync_flags: libc::c_uint,
) {
    _io_uring_prep_fsync(sqe, fd, fsync_flags);
}

pub unsafe fn io_uring_prep_nop(sqe: *mut io_uring_sqe) {
    _io_uring_prep_nop(sqe);
}

pub unsafe fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    _io_uring_cqe_seen(ring, cqe)
}
