use crate::inner_prelude::*;

pub fn handle(fb: &mut FigureBuilder) {
    handle_optimal(fb);
    handle_broccoli(fb);
}

pub fn handle_broccoli(fb: &mut FigureBuilder) {
    #[derive(Serialize)]
    struct Res {
        bench: f64,
        bench_par: f64,
        collect: f64,
        collect_par: f64,
    }
    impl Res {
        fn new(grow: f64, num_bots: usize) -> Res {
            let mut bot_inner: Vec<_> = (0..num_bots).map(|_| 0isize).collect();

            let bench = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                bench_closure(|| {
                    tree.find_colliding_pairs_mut(|a, b| {
                        **a.unpack_inner() += 1;
                        **b.unpack_inner() += 1;
                    });
                })
            };

            let bench_par = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                bench_closure(|| {
                    tree.find_colliding_pairs_mut_par(RayonJoin, |a, b| {
                        **a.unpack_inner() += 1;
                        **b.unpack_inner() += 1;
                    });
                })
            };

            let collect = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                bench_closure(|| {
                    let c = tree.collect_colliding_pairs(|a, b| {
                        *a += 1;
                        *b += 1;
                        Some(())
                    });
                    black_box(c);
                })
            };

            let collect_par = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                bench_closure(|| {
                    let c = tree.collect_colliding_pairs_par(RayonJoin, |a, b| {
                        *a += 1;
                        *b += 1;
                        Some(())
                    });
                    black_box(c);
                })
            };

            black_box(bot_inner);

            Res {
                bench,
                bench_par,
                collect,
                collect_par,
            }
        }
    }

    fb.make_graph(Args {
        filename: "broccoli_query",
        title: &format!(
            "Bench of query vs collect with abspiral({},n)",
            DEFAULT_GROW
        ),
        xname: "Number of Elements",
        yname: "Time in Seconds",
        plots: n_iter(0, 40_000)
            .map(|num_bots| (num_bots as f64, Res::new(DEFAULT_GROW, num_bots))),
        stop_values: &[],
    });
}

pub fn handle_optimal(fb: &mut FigureBuilder) {
    #[derive(Serialize)]
    struct Res {
        optimal: f64,
        optimal_par: f64,
    }
    impl Res {
        fn new(grow: f64, num_bots: usize) -> Res {
            let mut bot_inner: Vec<_> = (0..num_bots).map(|_| 0isize).collect();

            let optimal = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                let mut pairs = tree.collect_colliding_pairs(|_, _| Some(()));

                bench_closure(|| {
                    pairs.for_every_pair_mut(&mut bot_inner, |a, b, ()| {
                        *a += 1;
                        *b += 1;
                    });
                })
            };

            let optimal_par = {
                let mut base =
                    crate::support::make_tree_ref_ind(&mut bot_inner, grow, |a| a.to_f64n());
                let mut tree = base.build();

                let mut pairs = tree.collect_colliding_pairs_par(RayonJoin, |_, _| Some(()));

                bench_closure(|| {
                    pairs.for_every_pair_mut_par(RayonJoin, &mut bot_inner, |a, b, ()| {
                        *a += 1;
                        *b += 1;
                    });
                })
            };

            black_box(bot_inner);

            Res {
                optimal,
                optimal_par,
            }
        }
    }

    fb.make_graph(Args {
        filename: "optimal_query",
        title: &format!("Bench of optimal with abspiral({},n)", DEFAULT_GROW),
        xname: "Number of Elements",
        yname: "Time in Seconds",
        plots: n_iter(0, 40_000)
            .map(|num_bots| (num_bots as f64, Res::new(DEFAULT_GROW, num_bots))),
        stop_values: &[],
    });
}
