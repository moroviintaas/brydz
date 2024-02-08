
/*pub fn init_hash24_array(){
    let array = HashArrayNodeStore::init();
}*/

use criterion::{Criterion, criterion_group, criterion_main};
use brydz_dd::hash::{hash24::Hash24, HashArrayNodeStore, ranker::MoreCardsRanker};

pub fn bench_init_hash_24_8_array(c: &mut Criterion){
    c.bench_function("Benchmark create hash array", |b| b.iter(||{
        let _array = HashArrayNodeStore::<Hash24<3>, MoreCardsRanker, 0x1000000, 8>::init();
    }
    ));
}

criterion_group!(benches, bench_init_hash_24_8_array);
criterion_main!(benches);