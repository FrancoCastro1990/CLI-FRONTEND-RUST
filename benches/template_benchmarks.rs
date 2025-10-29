//! Performance benchmarks for template engine operations
//!
//! Run with: `cargo bench`

use cli_frontend::template_engine::HandlebarsRenderer;
use cli_frontend::template_engine::TemplateRenderer;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark case conversion through Handlebars helpers
fn benchmark_case_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("case_conversions");
    let renderer = HandlebarsRenderer::new();

    // PascalCase helper
    group.bench_function("pascal_case", |b| {
        let data = serde_json::json!({"name": "hello_world"});
        b.iter(|| renderer.render(black_box("{{pascal_case name}}"), black_box(&data)))
    });

    // snake_case helper
    group.bench_function("snake_case", |b| {
        let data = serde_json::json!({"name": "HelloWorld"});
        b.iter(|| renderer.render(black_box("{{snake_case name}}"), black_box(&data)))
    });

    // camelCase helper
    group.bench_function("camel_case", |b| {
        let data = serde_json::json!({"name": "hello_world"});
        b.iter(|| renderer.render(black_box("{{camel_case name}}"), black_box(&data)))
    });

    // kebab-case helper
    group.bench_function("kebab_case", |b| {
        let data = serde_json::json!({"name": "HelloWorld"});
        b.iter(|| renderer.render(black_box("{{kebab_case name}}"), black_box(&data)))
    });

    group.finish();
}

/// Benchmark full template rendering
fn benchmark_full_template_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_template_render");
    let renderer = HandlebarsRenderer::new();

    // Complete React component template
    let template = r#"
import React from 'react';

interface {{pascal_case name}}Props {
    className?: string;
}

export const {{pascal_case name}}: React.FC<{{pascal_case name}}Props> = ({ className }) => {
    return (
        <div className={`{{kebab_case name}} ${className || ''}`}>
            <h1>{{pascal_case name}}</h1>
        </div>
    );
};
"#;

    group.bench_function("react_component", |b| {
        let data = serde_json::json!({"name": "UserProfile"});
        b.iter(|| renderer.render(black_box(template), black_box(&data)))
    });

    group.finish();
}

/// Benchmark Handlebars rendering
fn benchmark_handlebars_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("handlebars_render");

    let renderer = HandlebarsRenderer::new();

    // Simple variable substitution
    group.bench_function("simple_variable", |b| {
        let data = serde_json::json!({"name": "World"});
        b.iter(|| renderer.render(black_box("Hello {{name}}!"), black_box(&data)))
    });

    // With helper
    group.bench_function("with_helper", |b| {
        let data = serde_json::json!({"name": "hello_world"});
        b.iter(|| renderer.render(black_box("{{pascal_case name}}"), black_box(&data)))
    });

    // Multiple helpers
    group.bench_function("multiple_helpers", |b| {
        let data = serde_json::json!({"name": "HelloWorld"});
        b.iter(|| {
            renderer.render(
                black_box("{{pascal_case name}} - {{snake_case name}} - {{kebab_case name}}"),
                black_box(&data),
            )
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_case_conversions,
    benchmark_full_template_render,
    benchmark_handlebars_render
);
criterion_main!(benches);
