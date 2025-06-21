use criterion::{criterion_group, criterion_main, Criterion};
use rust_demo::i18n1;
use rust_demo::i18n2;
use rust_demo::i18n3;
use rust_demo::i18n4;
use rust_demo::i18n5;
use rust_demo::Language;
use std::hint::black_box;

/// 结论：使用 AtomicU8Relaxed 和 AtomicU8SeqCst 没有太大的变化，只是 1.2 ns 和 1.5 ns 的区别而已
fn benchmark_get_lang_plan_a(c: &mut Criterion) {
    use i18n1::{get_lang, set_lang};

    set_lang(Language::tc);
    c.bench_function("AtomicU8Relaxed 1", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });

    set_lang(Language::en);
    c.bench_function("AtomicU8Relaxed 2", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });
}

fn benchmark_get_lang_plan_b(c: &mut Criterion) {
    use i18n2::{get_lang, set_lang};

    set_lang(Language::tc);
    c.bench_function("AtomicU8SeqCst 1", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });

    set_lang(Language::en);
    c.bench_function("AtomicU8SeqCst 2", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });
}

/// 结论：使用 transmute 相比使用 match 会快很多，一下子从 1.2 ns 减少到了 255.89 ps (0.255 ns)
fn benchmark_get_lang_plan_c(c: &mut Criterion) {
    use i18n3::{get_lang, set_lang};

    set_lang(Language::tc);
    c.bench_function("AtomicU8Relaxed transmute 1", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });

    set_lang(Language::en);
    c.bench_function("AtomicU8Relaxed transmute 2", |b| {
        b.iter(|| {
            black_box(get_lang());
        })
    });
}

fn benchmark_tr_plan_a(c: &mut Criterion) {
    use i18n3::set_lang;
    use i18n4::tr;

    set_lang(Language::tc);
    c.bench_function("Perfect Hash Function tc", |b| {
        b.iter(|| {
            for text in [
                "Settings",
                "Gold",
                "Door",
                "Revenant",
                "OutofMan",
                "Criticalh",
                "Criticalh",
                "Theancien",
                "Thekingdo",
            ] {
                black_box(tr(text));
            }
        })
    });

    set_lang(Language::en);
    c.bench_function("Perfect Hash Function en", |b| {
        b.iter(|| {
            for text in [
                "Settings",
                "Gold",
                "Door",
                "Revenant",
                "OutofMan",
                "Criticalh",
                "Criticalh",
                "Theancien",
                "Thekingdo",
            ] {
                black_box(tr(text));
            }
        })
    });
}

/// 结论：使用 i18n! 的方案比使用 phf_map! 的方案要快，phf_map! 方案在 170 ns 左右，而 i18n! 方案只需要 14 ns 左右
/// 并且 phf_map! 方案本身还有所限制的
fn benchmark_tr_plan_b(c: &mut Criterion) {
    use i18n3::set_lang;
    use i18n5::Text;

    set_lang(Language::tc);
    c.bench_function("Enum match with macro i18n! tc", |b| {
        b.iter(|| {
            for text in [
                Text::Settings,
                Text::Gold,
                Text::Door,
                Text::Revenant,
                Text::OutofMan,
                Text::Criticalh,
                Text::Criticalh,
                Text::Theancien,
                Text::Thekingdo,
            ] {
                black_box(text.text());
            }
        })
    });

    set_lang(Language::en);
    c.bench_function("Enum match with macro i18n! en", |b| {
        b.iter(|| {
            for text in [
                Text::Settings,
                Text::Gold,
                Text::Door,
                Text::Revenant,
                Text::OutofMan,
                Text::Criticalh,
                Text::Criticalh,
                Text::Theancien,
                Text::Thekingdo,
            ] {
                black_box(text.text());
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = benchmark_get_lang_plan_a, benchmark_get_lang_plan_b, benchmark_get_lang_plan_c, benchmark_tr_plan_a, benchmark_tr_plan_b
}
criterion_main!(benches);
