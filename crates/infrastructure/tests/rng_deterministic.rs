use infrastructure::rng::seeded_rng::SeededRng;
use contracts::rng::Rng;

#[test]
fn same_seed_generates_same_values() {
    let rng1 = SeededRng::new(42);
    let rng2 = SeededRng::new(42);

    assert_eq!(rng1.u32(1, 100), rng2.u32(1, 100));
}