use clorinde::{config::Config, conn::clorinde_conn, CodegenSettings};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    clorinde::container::cleanup(false).ok();
    clorinde::container::setup(false).unwrap();
    let client = &mut clorinde_conn().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    clorinde::load_schema(client, &["../test_codegen/schema.sql"]).unwrap();
    c.bench_function("codegen_sync", |b| {
        b.iter(|| {
            clorinde::gen_live(
                client,
                "../test_codegen/queries".as_ref(),
                tmp.path(),
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                    config: Config::default(),
                },
            )
            .unwrap()
        })
    });
    c.bench_function("codegen_async", |b| {
        b.iter(|| {
            clorinde::gen_live(
                client,
                "../test_codegen/queries".as_ref(),
                tmp.path(),
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                    config: Config::default(),
                },
            )
            .unwrap()
        })
    });
    clorinde::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
