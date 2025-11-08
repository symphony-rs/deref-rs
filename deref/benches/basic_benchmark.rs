use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use deref::{deref, deref_mut};

// 手动实现 Deref 的结构体
#[derive(Debug, Clone, Copy)]
struct ManualDeref {
    value: i32,
}

impl std::ops::Deref for ManualDeref {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// 使用 deref 宏实现 Deref 的结构体
#[derive(Debug, Clone, Copy)]
struct MacroDeref {
    value: i32,
}

deref!(MacroDeref, i32, value);

// 使用 deref_mut 宏实现 Deref 和 DerefMut 的结构体
#[derive(Debug, Clone, Copy)]
struct MacroDerefMut {
    value: i32,
}

deref_mut!(MacroDerefMut, i32, value);

// 手动实现 Deref 和 DerefMut 的结构体
#[derive(Debug, Clone, Copy)]
struct ManualDerefMut {
    value: i32,
}

impl std::ops::Deref for ManualDerefMut {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for ManualDerefMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn bench_deref(c: &mut Criterion) {
    let manual = ManualDeref { value: 42 };
    let macro_deref = MacroDeref { value: 42 };

    c.bench_function("manual_deref", |b| {
        b.iter(|| {
            let value = **black_box(&manual);
            black_box(value)
        })
    });

    c.bench_function("macro_deref", |b| {
        b.iter(|| {
            let value = **black_box(&macro_deref);
            black_box(value)
        })
    });
}

fn bench_deref_mut(c: &mut Criterion) {
    c.bench_function("manual_deref_mut", |b| {
        b.iter(|| {
            let mut manual = ManualDerefMut { value: 42 };
            **black_box(&mut manual) += 1;
            black_box(manual.value)
        })
    });

    c.bench_function("macro_deref_mut", |b| {
        b.iter(|| {
            let mut macro_deref_mut = MacroDerefMut { value: 42 };
            **black_box(&mut macro_deref_mut) += 1;
            black_box(macro_deref_mut.value)
        })
    });
}

fn bench_nested_deref(c: &mut Criterion) {
    // 嵌套结构体测试
    #[derive(Debug, Clone, Copy)]
    struct Inner {
        value: i32,
    }

    #[derive(Debug, Clone, Copy)]
    struct OuterManual {
        inner: Inner,
    }

    impl std::ops::Deref for OuterManual {
        type Target = Inner;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct OuterMacro {
        inner: Inner,
    }

    deref!(OuterMacro, Inner, inner);

    let manual = OuterManual {
        inner: Inner { value: 42 },
    };
    let macro_deref = OuterMacro {
        inner: Inner { value: 42 },
    };

    c.bench_function("nested_manual_deref", |b| {
        b.iter(|| {
            let value = black_box(&manual).value;
            black_box(value)
        })
    });

    c.bench_function("nested_macro_deref", |b| {
        b.iter(|| {
            let value = black_box(&macro_deref).value;
            black_box(value)
        })
    });
}

criterion_group!(benches, bench_deref, bench_deref_mut, bench_nested_deref);
criterion_main!(benches);
