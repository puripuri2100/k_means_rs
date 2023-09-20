//! Rustでのk_meansアルゴリズムの実装例
//!
//! ---
//!
//! The MIT License
//!
//! Copyright (c) 2023 Naoki Kaneko (a.k.a. "puripuri2100")
//!

use anyhow::Result;
use plotters::prelude::*;
use rand::Rng;
use tokio_stream::StreamExt;

mod k_means;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct T {
  x: usize,
  y: usize,
}

fn calc_distance(t1: &T, t2: &T) -> usize {
  ((((t1.x - t2.x).pow(2) + (t1.y - t2.y).pow(2)) as f64).sqrt()) as usize
}

fn calc_center(lst: &[T]) -> T {
  let len = lst.len();
  let mut x_sum = 0;
  let mut y_sum = 0;
  for t in lst.iter() {
    x_sum += t.x;
    y_sum += t.y;
  }
  T {
    x: x_sum / len,
    y: y_sum / len,
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let mut rng = rand::thread_rng();

  // 座標
  let mut lst = Vec::new();
  let mut stream = tokio_stream::iter(0..800);
  while stream.next().await.is_some() {
    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);
    let t = T { x, y };
    lst.push(t)
  }

  // 重心
  let mut center_lst = Vec::new();
  let mut center_stream = tokio_stream::iter(0..3);
  while center_stream.next().await.is_some() {
    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);
    let t = T { x, y };
    center_lst.push(t)
  }

  let solved = k_means::solve(calc_distance, calc_center, center_lst, &lst).await;

  let root = BitMapBackend::new("test.png", (1024, 1024)).into_drawing_area();

  root.fill(&WHITE)?;

  let areas = root.split_by_breakpoints([944], [80]);
  let mut scatter_ctx = ChartBuilder::on(&areas[2])
    .x_label_area_size(100)
    .y_label_area_size(100)
    .build_cartesian_2d(0..100, 0..100)?;
  scatter_ctx.draw_series(
    solved[0]
      .iter()
      .map(|T { x, y }| Circle::new((*x as i32, *y as i32), 2, GREEN.filled())),
  )?;
  scatter_ctx.draw_series(
    solved[1]
      .iter()
      .map(|T { x, y }| Circle::new((*x as i32, *y as i32), 2, RED.filled())),
  )?;
  scatter_ctx.draw_series(
    solved[2]
      .iter()
      .map(|T { x, y }| Circle::new((*x as i32, *y as i32), 2, BLUE.filled())),
  )?;

  root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
  Ok(())
}
