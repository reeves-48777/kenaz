use criterion::{Criterion, criterion_group, criterion_main};
use kenaz_core::{
    Engram,
    engram::{
        fit::{self, ExtractedColor},
        vector::{EngramVector, OpType},
    },
    palette::Colors,
    schema::ThemeStyleContent,
    visitor::ColorMutable,
};
use std::hint::black_box;

// Dummy anchors
fn dummy_anchors() -> Colors {
    Colors {
        bg: ExtractedColor::parse_hex("#1e1e2e").unwrap().oklab,
        fg: ExtractedColor::parse_hex("#cdd6f4").unwrap().oklab,
        accent: ExtractedColor::parse_hex("#89b4fa").unwrap().oklab,
        success: ExtractedColor::parse_hex("#a6e3a1").unwrap().oklab,
        warning: ExtractedColor::parse_hex("#f9e2af").unwrap().oklab,
        error: ExtractedColor::parse_hex("#f38ba8").unwrap().oklab,
    }
}

fn dummy_engram() -> Engram {
    let mut map = std::collections::HashMap::new();
    let v = EngramVector {
        op_type: OpType::Direct,
        weights: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        delta_l: 0.0,
        alpha: 1.0,
    };

    for path in [
        "background",
        "text",
        "editor_background",
        "editor_foreground",
        "syntax_keyword",
    ] {
        map.insert(path.to_string(), v);
    }

    map
}

fn dummy_style() -> ThemeStyleContent {
    let json_str = r##"{
        "background": "#1e1e2e",
        "text": "#cdd6f4",
        "text.accent": "#89b4fa",
        "editor": {
            "background": "#1e1e2e",
            "foreground": "#cdd6f4"
        },
        "syntax": {
            "keyword": {
                "color": "#89b4fa"
            }
        }
    }"##;

    serde_json::from_str(json_str).expect("Failed to parse dummy style")
}

fn bench_fit_token(c: &mut Criterion) {
    let anchors = dummy_anchors();

    let target = ExtractedColor::parse_hex("#ff5500").unwrap();

    c.bench_function("fit_token", |b| {
        b.iter(|| fit::fit_token(black_box(target), black_box(&anchors)))
    });
}

fn bench_apply_colors(c: &mut Criterion) {
    let anchors = dummy_anchors();
    let engram = dummy_engram();
    let mut style = dummy_style();

    c.bench_function("apply_colors", |b| {
        b.iter(|| {
            let mut path_buf = String::with_capacity(64);
            black_box(&mut style).apply_colors(
                black_box(&mut path_buf),
                black_box(&engram),
                black_box(&anchors),
            );
        })
    });
}

criterion_group!(benches, bench_fit_token, bench_apply_colors);
criterion_main!(benches);
