#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct TestData {
    vec: Vec<u8>,
    string: String,
    num: i32,
}

fuzz_target!(|data: TestData| {
    // Test lease_mut on Vec with various inputs
    let mut v = data.vec.clone();
    let _ = lease::lease_mut(&mut v, |mut owned| {
        owned.push(42);
        // Use the string and num fields to create varied behavior
        if data.string.len() > 10 {
            owned.push(data.num as u8);
        }
        if data.num > 100 {
            owned.extend_from_slice(&[1, 2, 3]);
        }
        (owned, ())
    });

    // Test try_lease_mut with error path
    let mut v2 = data.vec.clone();
    let _ = lease::try_lease_mut(&mut v2, |mut owned| {
        owned.push(99);
        if owned.len() > 100 || data.num > 1000 {
            (owned, Err("too big"))
        } else {
            (owned, Ok(()))
        }
    });

    // Test with different data types and operations
    let mut v3 = data.vec.clone();
    let _ = lease::lease_mut(&mut v3, |mut owned| {
        // Various operations that might trigger edge cases
        owned.push(data.num as u8);
        if owned.len() > 50 {
            owned.truncate(10);
        }
        if data.string.contains('x') {
            owned.reverse();
        }
        (owned, ())
    });
});