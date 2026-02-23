#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use futures::executor::block_on;

#[derive(Arbitrary, Debug)]
struct TestData { vec: Vec<u8> }

fuzz_target!(|data: TestData| {
    let mut v = data.vec.clone();

    // Test the checked path (clone + guard)
    let _ = block_on(async {
        let _ = lease::lease_async_mut(&mut v, |mut owned| async move {
            owned.push(42);
            (owned, Ok::<(), ()>(()))
        }).await;
    });

    // Test unchecked (should work normally when not cancelled)
    let mut v2 = data.vec.clone();
    let _ = block_on(async {
        let _ = lease::lease_async_mut_unchecked(&mut v2, |mut owned| async move {
            owned.push(99);
            (owned, ())
        }).await;
    });
});