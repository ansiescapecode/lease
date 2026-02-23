#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct TestData { vec: Vec<u8> }

fuzz_target!(|data: TestData| {
    let _ = lease_rs::lease(data.vec.clone(), |mut v| {
        v.push(42);
        (v, ())
    });

    let _: Result<_, &str> = lease_rs::try_lease(data.vec, |mut v| {
        v.push(99);
        Ok((v, ()))
    });
});