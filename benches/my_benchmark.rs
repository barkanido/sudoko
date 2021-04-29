use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku::{solve_sudoku, Sudoku};

fn criterion_benchmark(c: &mut Criterion) {
    let expert_board = vec![
        0, 0, 5, 0, 0, 0, 0, 6, 2, 0, 6, 3, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        0, 0, 6, 7, 0, 3, 0, 0, 6, 7, 0, 5, 0, 0, 0, 1, 0, 0, 8, 0, 0, 0, 0, 0, 8, 0, 1, 2, 0, 0,
        6, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 0, 0, 4, 0, 0, 0, 0, 8, 0, 0,
    ];
    // let easy_board = vec![
    //     0, 0, 0, 5, 9, 0, 0, 3, 7, 0, 7, 9, 0, 3, 2, 0, 8, 0, 0, 0, 8, 7, 0, 0, 0, 0, 0, 3, 0, 0,
    //     0, 0, 1, 6, 2, 0, 0, 9, 0, 0, 0, 6, 3, 7, 0, 7, 2, 0, 3, 0, 0, 1, 5, 8, 0, 0, 0, 0, 0, 7,
    //     8, 6, 5, 0, 8, 7, 0, 1, 0, 2, 4, 0, 0, 5, 4, 6, 0, 0, 7, 0, 0,
    // ];
    let mut game = Sudoku::from(expert_board);
    // let mut game = Sudoku::from(expert_board);
    // println!("{}", game);
    // solve_sudoku(&mut game);
    // println!("{}", game);
    c.bench_function("sudoko expert", |b| {
        b.iter(|| solve_sudoku(black_box(&mut game)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
