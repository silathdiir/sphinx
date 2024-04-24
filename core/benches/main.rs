use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wp1_core::io::SP1Stdin;
use wp1_core::runtime::{Program, Runtime};
use wp1_core::utils::{run_and_prove, BabyBearPoseidon2};

#[allow(unreachable_code)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("prove");
    group.sample_size(10);
    let programs = ["fibonacci"];
    for p in programs {
        let elf_path = format!("../programs/demo/{}/elf/riscv32im-succinct-zkvm-elf", p);
        let program = Program::from_elf(&elf_path);
        let cycles = {
            let mut runtime = Runtime::new(program.clone());
            runtime.run();
            runtime.state.global_clk
        };
        group.bench_function(
            format!("main:{}:{}", p.split('/').last().unwrap(), cycles),
            |b| {
                b.iter(|| {
                    run_and_prove(
                        black_box(&program),
                        &SP1Stdin::new(),
                        BabyBearPoseidon2::new(),
                    )
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
