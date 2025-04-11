use criterion::criterion_main;
use crate::cosmic::cosmic_benches;
use crate::parley::parley_benches;

pub mod cosmic;
pub mod parley;

criterion_main!(cosmic_benches, parley_benches);