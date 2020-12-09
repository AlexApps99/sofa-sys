#[test]
fn check_linked() {
    unsafe {
        sofa_sys::iauEect00(0.0, 0.0);
    }
    // It's linked!
}
