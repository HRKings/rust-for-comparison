use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! cfor {
  ($var:stmt; $test:expr; $increment:expr, $body:block) => {
      {
          $var
          while ($test) {
              $body
              $increment;
          }
      }
  }
}

fn cfor(b: &mut Criterion) {
    b.bench_function("cfor", |b| b.iter(|| {
        let mut control = 0;

        cfor!(let mut i=0; i < 10; i += 1, {
            control += black_box(1);
        });

        control
    }));

    b.bench_function("cfor_blackbox", |b| b.iter(|| {
      let mut control = 0;

      cfor!(let mut i=0; i < 10; i += black_box(1), {
          control += black_box(1);
      });

      control
  }));
}
fn for_each(b: &mut Criterion) {
    b.bench_function("for_each", |b| b.iter(|| {
        let mut for_each = 0;

        (0..10).for_each(|_| {
            for_each += black_box(1);
        });

        for_each
    }));

    b.bench_function("for_each_blackbox", |b| b.iter(|| {
      let mut for_each = 0;

      (0..black_box(10)).for_each(|_| {
          for_each += black_box(1);
      });

      for_each
  }));
}

fn for_range(b: &mut Criterion) {
    b.bench_function("for_range", |b| b.iter(|| {
        let mut range_for = 0;

        for i in 0..10 {
            range_for += black_box(1);
        }

        range_for
    }));

    b.bench_function("for_range_blackbox", |b| b.iter(|| {
      let mut range_for = 0;

      for i in 0..black_box(10) {
          range_for += black_box(1);
      }

      range_for
  }));
}

criterion_group!(benches, cfor, for_each, for_range);
criterion_main!(benches);
