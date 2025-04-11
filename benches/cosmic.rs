use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, Wrap};
use criterion::{black_box, criterion_group, Criterion};
fn cosmic_load_font_system(c: &mut Criterion) {
    c.bench_function("load FontSystem", |b| {
        b.iter(|| black_box(FontSystem::new()))
    });
}

fn cosmic_layout(c: &mut Criterion) {
    let mut fs = FontSystem::new();
    let mut buffer = Buffer::new(&mut fs, Metrics::new(10.0, 10.0));
    buffer.set_size(&mut fs, Some(80.0), None);
    
    let mut group = c.benchmark_group("Cosmic Layout");
    buffer.set_wrap(&mut fs, Wrap::Word);

    let mut run_on_text = |text: &str| {
        buffer.lines.clear();
        buffer.set_text(&mut fs, text, &Attrs::new(), Shaping::Advanced);
        buffer.shape_until_scroll(&mut fs, false);
    };

    group.bench_function("small amount of text", |b| {
        b.iter(|| {
            run_on_text("Hello, world!");
        });
    });

    group.bench_function("large amount of text", |b| {
        b.iter(|| {
            run_on_text(include_str!("../sample/FIRST_CHAPTER_OF_MOBY_DICK.txt"));
        });
    });

    group.bench_function("arabic text", |b| {
        b.iter(|| {
            run_on_text(include_str!("../sample/arabic.txt"));
        })
    });

    // Reduce the sample count for these next ones.
    // If we can optimize the layout for these, remove this line.
    group.sample_size(10);

    group.bench_function("hebrew text", |b| {
        b.iter(|| {
            run_on_text(include_str!("../sample/hebrew.txt"));
        })
    });

    group.bench_function("emoji text", |b| {
        b.iter(|| {
            run_on_text(include_str!("../sample/emoji.txt"));
        })
    });
    
}

criterion_group!(cosmic_benches, cosmic_layout, cosmic_load_font_system);
