use std::time::Instant;

struct WorkItem {
    strings: Vec<String>,
}

fn process_with_chili<'s, T: Send + Sync + 'static, O: Send + Sync + 'static>(
    scope: &mut chili::Scope<'s>,
    work: &[T],
    out: &mut [O],
    cb: impl Fn(&T) -> O + Send + Sync + Copy,
) {
    let len = work.len();
    if len == 1 {
        out[0] = cb(&work[0]);
        return;
    }
    let mid = len / 2;
    let (a_in, b_in) = work.split_at(mid);
    let (a_out, b_out) = out.split_at_mut(mid);
    scope.join(
        |scope| process_with_chili(scope, a_in, a_out, cb),
        |scope| process_with_chili(scope, b_in, b_out, cb),
    );
}

fn main() {
    let large_at = 400..440;
    let work_items: Vec<WorkItem> = (1..1000)
        .map(|i| {
            let size = if large_at.contains(&i) {
                200_000
            } else {
                10_000
            };

            WorkItem {
                strings: (0..size).map(|x| x.to_string()).collect(),
            }
        })
        .collect();

    let chili_tp = chili::ThreadPool::new();
    let mut chili_scope = chili_tp.scope();

    let mut paralight_tp = paralight::ThreadPoolBuilder {
        num_threads: paralight::ThreadCount::AvailableParallelism,
        range_strategy: paralight::RangeStrategy::WorkStealing,
        cpu_pinning: paralight::CpuPinningPolicy::No,
    }
    .build();

    for _ in 0..5 {
        println!("====");

        // chili
        {
            let start = Instant::now();
            let mut out: Vec<usize> = vec![0; work_items.len()];
            process_with_chili(&mut chili_scope, &work_items, &mut out, |item| {
                let mut x: Vec<String> = item.strings.to_vec();
                x.sort();
                x.first().unwrap().len()
            });
            assert_eq!(out[9], 1);

            println!("chili: {}", start.elapsed().as_millis());
        }

        // rayon
        {
            use rayon::iter::IntoParallelRefIterator as _;
            use rayon::iter::ParallelIterator as _;
            let start = Instant::now();
            let out: Vec<usize> = work_items
                .par_iter()
                .map(|item| {
                    let mut x: Vec<String> = item.strings.to_vec();
                    x.sort();
                    x.first().unwrap().len()
                })
                .collect();

            assert_eq!(out[9], 1);

            println!("rayon: {}", start.elapsed().as_millis());
        }

        // orx-parallel
        {
            use orx_parallel::*;

            let chunk_sizes = [0, 1, 32, 64];

            for chunk_size in chunk_sizes {
                let start = Instant::now();
                let out: Vec<usize> = work_items
                    .par()
                    .map(|item| {
                        let mut x: Vec<String> = item.strings.to_vec();
                        x.sort();
                        x.first().unwrap().len()
                    })
                    .chunk_size(chunk_size)
                    .collect();

                assert_eq!(out[9], 1);

                println!("orx-parallel-{chunk_size}: {}", start.elapsed().as_millis());
            }
        }

        // paralight
        {
            use paralight::iter::IntoParallelRefMutSource as _;
            use paralight::iter::IntoParallelRefSource as _;
            use paralight::iter::ParallelIteratorExt as _;
            use paralight::iter::ParallelSourceExt as _;
            use paralight::iter::ZipableSource as _;

            let start = Instant::now();
            let mut out: Vec<usize> = vec![0; work_items.len()];
            (out.par_iter_mut(), work_items.par_iter())
                .zip_eq()
                .with_thread_pool(&mut paralight_tp)
                .for_each(|(o, item)| {
                    let mut x: Vec<String> = item.strings.to_vec();
                    x.sort();
                    *o = x.first().unwrap().len();
                });

            assert_eq!(out[9], 1);

            println!("paralight: {}", start.elapsed().as_millis());
        }
    }
}
