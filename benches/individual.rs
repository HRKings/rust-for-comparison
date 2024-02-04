use criterion::{black_box, criterion_group, criterion_main, Criterion};

use for_comparison::cfor;

fn cfor(c: &mut Criterion) {
    let mut g = c.benchmark_group("cfor");

    g.bench_function("cfor_base", |b| b.iter(|| {
        let mut control = 0;

        cfor!(let mut i=0; i < 10; i += 1, {
            control += black_box(1);
        });

        control
    }));

    g.bench_function("cfor_blackbox", |b| b.iter(|| {
      let mut control = 0;

      cfor!(let mut i=0; i < black_box(10); i += black_box(1), {
          control += black_box(1);
      });

      control
    }));

    g.bench_function("cfor_step", |b| b.iter(|| {
        let mut control = 0;

        cfor!(let mut i=0; i < 10; i += 4, {
            control += black_box(1);
        });

        control
    }));

    g.bench_function("cfor_step_blackbox", |b| b.iter(|| {
      let mut control = 0;

      cfor!(let mut i=0; i < black_box(10); i += black_box(4), {
          control += black_box(1);
      });

      control
    }));

    g.finish();
}

fn cfor_inline(c: &mut Criterion) {
    let mut g = c.benchmark_group("cfor_inline");

    g.bench_function("cfor_inline_base", |b| b.iter(|| {
        let mut control = 0;

        let mut i=0;
        while i < 10 {
            control += black_box(1);
            i += 1;
        }

        control
    }));

    g.bench_function("cfor_inline_blackbox", |b| b.iter(|| {
      let mut control = 0;

      let mut i=0; 
       while i < black_box(10) {
          control += black_box(1);
          i += black_box(1);
      }

      control
    }));

    g.bench_function("cfor_inline_step", |b| b.iter(|| {
        let mut control = 0;

        let mut i=0; 
        while i < 10 {
            control += black_box(1);
            i += 4;
        }

        control
    }));

    g.bench_function("cfor_inline_step_blackbox", |b| b.iter(|| {
      let mut control = 0;

      let mut i=0;
      while i < black_box(10) {
          control += black_box(1);
          i += black_box(4);
      }

      control
    }));

    g.finish();
}

fn for_each(c: &mut Criterion) {
    let mut g = c.benchmark_group("for_each");
    
    g.bench_function("for_each_base", |b| b.iter(|| {
        let mut for_each = 0;

        (0..10).for_each(|_| {
            for_each += black_box(1);
        });

        for_each
    }));

    g.bench_function("for_each_blackbox", |b| b.iter(|| {
      let mut for_each = 0;

      (0..black_box(10)).for_each(|_| {
          for_each += black_box(1);
      });

      for_each
    }));

    g.bench_function("for_each_step", |b| b.iter(|| {
        let mut for_each = 0;

        (0..10).step_by(4).for_each(|_| {
            for_each += black_box(1);
        });

        for_each
    }));

    g.bench_function("for_each_step_blackbox", |b| b.iter(|| {
      let mut for_each = 0;

      (0..black_box(10)).step_by(4).for_each(|_| {
          for_each += black_box(1);
      });

      for_each
    }));

    g.finish();
}

fn for_range(c: &mut Criterion) {
    let mut g = c.benchmark_group("for_range");

    g.bench_function("for_range_base", |b| b.iter(|| {
        let mut range_for = 0;

        for _i in 0..10 {
            range_for += black_box(1);
        }

        range_for
    }));

    g.bench_function("for_range_blackbox", |b| b.iter(|| {
      let mut range_for = 0;

      for _i in 0..black_box(10) {
          range_for += black_box(1);
      }

      range_for
    }));

    g.bench_function("for_range_step", |b| b.iter(|| {
        let mut range_for = 0;

        for _i in (0..10).step_by(4) {
            range_for += black_box(1);
        }

        range_for
    }));

    g.bench_function("for_range_step_blackbox", |b| b.iter(|| {
        let mut range_for = 0;

        for _i in (0..black_box(10)).step_by(4) {
            range_for += black_box(1);
        }

        range_for
    }));

    g.finish();
}

criterion_group!(benches, cfor, cfor_inline, for_each, for_range);
criterion_main!(benches);
