use slab_alloc::Slab;

#[test]
fn basic_usage() {
    let mut slab = Slab::new();

    let x = slab.allocate(42).unwrap();
    assert_eq!(*x, 42);

    let ptr = x as *mut _;
    slab.deallocate(ptr);

    let y = slab.allocate(99).unwrap();
    assert_eq!(*y, 99);
}
