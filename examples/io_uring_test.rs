use std::env;
use std::fs::File;
use std::io::Error;
use std::mem;
use std::os::unix::io::AsRawFd;

use libc::posix_memalign;
use liburing_sys::*;

const QD: u32 = 4;

fn main() {
    let mut ring: io_uring = unsafe { mem::uninitialized() };

    let ret = unsafe { io_uring_queue_init(QD, &mut ring, 0) };

    if ret < 0 {
        panic!("ret: {:?}", Error::from_raw_os_error(ret));
    }

    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("{}: file", args[0]);
        return;
    }
    let file = File::open(&args[1]).unwrap();

    let mut iovecs: Vec<libc::iovec> = vec![unsafe { mem::zeroed() }; QD as usize];
    for iov in iovecs.iter_mut() {
        let mut buf = unsafe { mem::uninitialized() };
        if unsafe { posix_memalign(&mut buf, 4096, 4096) } != 0 {
            panic!("can't allocate");
        }
        iov.iov_base = buf;
        iov.iov_len = 4096;
    }

    let mut offset: usize = 0;
    for i in 0.. {
        let sqe = unsafe { io_uring_get_sqe(&mut ring) };
        if sqe == std::ptr::null_mut() {
            break;
        }
        unsafe { io_uring_prep_readv(sqe, file.as_raw_fd(), &mut iovecs[i], 1, offset as i64) };
        offset += iovecs[i].iov_len;
    }
    let ret = unsafe { io_uring_submit(&mut ring) };
    if ret < 0 {
        panic!("ret: {:?}", Error::from_raw_os_error(ret));
    }

    let mut cqe: *mut io_uring_cqe = unsafe { std::mem::zeroed() };
    let mut done = 0;
    let pending = ret;
    for _ in 0..pending {
        let ret = unsafe { io_uring_wait_cqe(&mut ring, &mut cqe) };
        if ret < 0 {
            panic!("ret: {:?}", Error::from_raw_os_error(ret));
        }
        done += 1;
        let mut stop = false;
        if unsafe { (*cqe).res } != 4096 {
            eprintln!("res = {}, wanted 4096", unsafe { (*cqe).res });
            stop = true;
        }
        unsafe { io_uring_cqe_seen(&mut ring, cqe) };
        if stop {
            break;
        }
    }
    println!("Submitted={}, completed={}", pending, done);
    unsafe { io_uring_queue_exit(&mut ring) };
}
