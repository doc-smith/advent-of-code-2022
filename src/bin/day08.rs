use std::io;
use std::ops::{Index, IndexMut};

struct Grid<T> {
    w: usize,
    h: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone
{
    fn from_rows(rows: &[Vec<T>]) -> Self {
        let h = rows.len();
        let mut data = Vec::new();
        if h == 0 {
            return Grid { w: 0, h, data };
        }
        let w = rows[0].len();
        for row in rows {
            assert_eq!(row.len(), w);
            data.extend_from_slice(row);
        }
        Grid { w, h, data }
    }
}

impl<T> Grid<T>
where
    T: Clone + Default
{
    fn zeros_like<U>(other: &Grid<U>) -> Self {
        Grid {
            w: other.w,
            h: other.h,
            data: vec![T::default(); other.w * other.h],
        }
    }
}

impl<T> Grid<T> {
    fn w(&self) -> usize {
        self.w
    }

    fn h(&self) -> usize {
        self.h
    }

    fn get(&self, i: usize, j: usize) -> Option<&T>  {
        self.get_index(i, j).map(|flat_i| &self.data[flat_i])
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.get_index(i, j).map(|flat_i| &mut self.data[flat_i])
    }

    fn get_index(&self, i: usize, j: usize) -> Option<usize> {
        if i < self.h && j < self.w {
            Some(i * self.w + j)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.get(i, j).expect("index out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.get_mut(i, j)
            .expect("indices are out of bounds")
    }
}

fn read_input() -> Grid<u8> {
    let rows: Vec<Vec<_>> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("input error")
                .into_bytes()
        })
        .collect();
    Grid::from_rows(&rows)
}

struct TreeHeightMeasurer {
    current_max: u8
}

impl TreeHeightMeasurer {
    fn new() -> Self {
        TreeHeightMeasurer { current_max: 0 }
    }

    fn measure_and_update(&mut self, height: u8, visible: &mut bool) {
        if height > self.current_max {
            self.current_max = height;
            *visible = true;
        }
    }
}

fn part_one(forest: &Grid<u8>) -> usize {
    let mut visible: Grid<bool> = Grid::zeros_like(forest);

    assert_eq!(forest.w(), visible.w());

    for i in 0..forest.h() {
        let mut lr = TreeHeightMeasurer::new();
        let mut rl = TreeHeightMeasurer::new();
        let mut tb = TreeHeightMeasurer::new();
        let mut bt = TreeHeightMeasurer::new();
        for j in 0..forest.w() {
            lr.measure_and_update(forest[(i, j)], &mut visible[(i, j)]);
            rl.measure_and_update(forest[(i, forest.w() - j - 1)], &mut visible[(i, forest.w() - j - 1)]);
            tb.measure_and_update(forest[(j, i)], &mut visible[(j, i)]);
            bt.measure_and_update(forest[(forest.h() - j - 1, i)], &mut visible[(forest.h() - j - 1, i)]);
        }
    }

    let mut cnt = 0;
    for i in 0..visible.h() {
        for j in 0..visible.w() {
            if visible[(i, j)] {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let grid = read_input();
    println!("{}", part_one(&grid));
}