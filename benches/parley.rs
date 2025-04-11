use criterion::{black_box, criterion_group, Criterion};
use parley::{Alignment, AlignmentOptions, FontContext, FontStack, Layout, LayoutContext, TextStyle};

fn parley_load_font_context(c: &mut Criterion) {
    c.bench_function("load FontContext", |b| {
        b.iter(|| {
            black_box(FontContext::new());
        })
    });
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
struct ColorBrush {
    color: [u8; 4],
}

fn parley_layout(c: &mut Criterion) {
    let font_size: f32 = 10.0;
    let line_height_multiplier: f32 = 1.0;
    
    let mut font_cx = FontContext::new();
    let mut layout_cx = LayoutContext::new();
    let buffer_width = 80.0f32;

    let text_color = [0, 0, 0, 255];
    let text_brush = ColorBrush { color: text_color };
    let font_stack = FontStack::from("system-ui");
    
    let root_style = TextStyle {
        brush: text_brush,
        font_stack,
        line_height: line_height_multiplier,
        font_size,
        ..Default::default()
    };

    let mut group = c.benchmark_group("Parley Layout");
    let mut run_on_text = |text: &str| {
        font_cx.collection.clear();

        let mut builder = layout_cx.tree_builder(&mut font_cx, 1.0, &root_style);
        builder.push_text(&text);
        
        let (mut layout, _text): (Layout<ColorBrush>, String) = builder.build();

        // Perform layout (including bidi resolution and shaping) with start alignment
        layout.break_all_lines(Some(buffer_width));
        layout.align(Some(buffer_width), Alignment::Start, AlignmentOptions::default());
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

criterion_group!(parley_benches, parley_layout, parley_load_font_context);
