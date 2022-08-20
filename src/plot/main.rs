use crate::kde::make_kde;
use crate::plot::colors::*;
use crate::plot::utils::*;
use crate::types::*;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::ops::Range;

pub const COLORS: [RGBColor; 4] = [
    RGBColor(76, 114, 176),
    RGBColor(221, 132, 82),
    RGBColor(85, 168, 104),
    RGBColor(196, 78, 82),
];

pub fn plot_kde_separate(
    all_station_stairs: &Vec<StationStairs>,
    train_passengers: &Vec<Vec<f64>>,
    multiplier: f64,
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    let kdes = train_passengers.iter().map(|tp| make_kde(multiplier, tp));

    abstract_plot!(
        "out/out.png",
        0.0..0.06,
        all_station_stairs,
        |i, chart: &mut Chart| {
            for (idx, kde) in kdes.clone().enumerate() {
                let color = if i == idx {
                    let c: RGBColor = COLORS[i];
                    c.stroke_width(2)
                } else {
                    GRAY.filled()
                };
                chart.draw_series(LineSeries::new(kde, color)).unwrap();
            }
        }
    )
}

pub fn plot_kde_together(
    all_station_stairs: &Vec<StationStairs>,
    train_passengers: &Vec<Vec<f64>>,
    filename: &'static str,
    multiplier: f64,
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = basic_chart!(&root)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..0.06_f64)?;

    let mut mesh = chart.configure_mesh();
    mesh.y_desc("frequency")
        .axis_desc_style((20).into_text_style(&root))
        .light_line_style(&WHITE)
        .x_desc("xpos")
        .draw()?;

    for ((this_station_stair, train_passenger), color) in
        all_station_stairs.iter().zip(train_passengers).zip(COLORS)
    {
        let res = make_kde(multiplier, train_passenger);
        chart
            .draw_series(LineSeries::new(res, color.stroke_width(3)))?
            .label(&this_station_stair.station_name)
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 6), (x + 12, y + 6)], color.filled())
            });

        plot_platform_bounds(&chart, &root, 0, 0)?;
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(&BLACK.mix(0.5))
        .legend_area_size(22)
        .label_font(("Hiragino Sans GB W3", 20))
        .draw()?;

    Ok(root.clone())
}

pub fn plot_strip(
    all_station_stairs: &Vec<StationStairs>,
    train_passengers: &Vec<Vec<f64>>,
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    abstract_plot!(
        "out/strip.png",
        0.0..1.0,
        all_station_stairs,
        |i, chart: &mut Chart| {
            let xs: &Vec<f64> = &train_passengers[i];
            let uniform = Uniform::new(0.0, 1.0_f64);
            let ys = rand::thread_rng().sample_iter(uniform).take(xs.len());

            chart
                .draw_series(
                    xs.iter()
                        .zip(ys)
                        .map(|(x, y)| {
                            Circle::new((*x, y), 2_i32, BLUE.filled())
                        })
                        .choose_multiple(&mut rand::thread_rng(), 200),
                )
                .unwrap();
        }
    )
}

pub fn plot_pdfs(
    filename: &str,
    all_station_stairs: &Vec<StationStairs>,
    pdfs: Vec<Vec<(f64, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let roots = root.split_evenly((pdfs.len(), 1));

    for ((pdf, r), station) in pdfs.iter().zip(roots).zip(all_station_stairs) {
        let mut chart =
            chart_with_mesh_and_ydesc!(&r, 0.0..2.0_f64, &station.station_name);
        chart
            .draw_series(LineSeries::new(pdf.clone(), BLUE.stroke_width(2)))?;
        plot_platform_bounds(&chart, &r, 0, 35)?;
    }

    Ok(())
}

pub fn plot_stair_pdfs_sep(
    filename: &str,
    pdfs: Vec<Vec<(f64, (f64, f64, f64))>>,
    stairs: &Vec<f64>,
    prev_pdf: &Vec<(f64, f64)>,
    this_pdf: &Vec<(f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let n_stairs = stairs.len();

    let root =
        BitMapBackend::new(filename, (1024 * 2, 1500)).into_drawing_area();
    root.fill(&WHITE)?;

    let (left, right) = root.split_horizontally(1024_i32);
    let right_roots = right.split_evenly((n_stairs + 2, 1));
    let roots = left.split_evenly((n_stairs + 2, 1));

    let as_: [fn((f64, f64, f64)) -> f64; 3] =
        [|ys| ys.0, |ys| ys.1, |ys| ys.2];

    for (idx, r) in roots.iter().enumerate().take(n_stairs) {
        r.titled(
            &format!("Passengers boarding at Ochanomizu stair #{}", idx + 1),
            ("sans-serif", 30_i32),
        )?;
        let mut chart = chart_with_mesh!(r, 0.0..1.5_f64);
        let labels = ["beta_far", "beta_close", "uniform"];
        for ((a, color), label) in as_.iter().zip(COLORS).zip(labels) {
            let pdf = &pdfs.iter().map(|x| x[idx]).map(|(x, ys)| (x, a(ys)));
            chart
                .draw_series(LineSeries::new(
                    pdf.clone(),
                    color.stroke_width(2),
                ))
                .unwrap()
                .label(label)
                .add_legend_icon(color);
        }

        plot_platform_bounds(&chart, r, 0, 35).unwrap();

        plot_stairs(r, &chart, stairs[idx], 0, 35).unwrap();

        if idx == 0 {
            add_legend!(chart).unwrap();
        }
    }

    let sum_pdfs: Vec<Vec<(f64, f64)>> = right_roots
        .iter()
        .enumerate()
        .take(n_stairs)
        .map(|(idx, r)| {
            r.titled(
                &format!("S_{}/{}", idx + 1, n_stairs),
                ("sans-serif", 30_i32),
            )
            .unwrap();
            let mut chart = chart_with_mesh!(r, 0.0..0.6_f64);
            let sum_pdf = &pdfs
                .iter()
                .map(|x| x[idx])
                .map(|(x, (a, b, c))| (x, (a + b + c) / stairs.len() as f64));

            chart
                .draw_series(LineSeries::new(
                    sum_pdf.clone(),
                    BLUE.stroke_width(2),
                ))
                .unwrap();

            plot_platform_bounds(&chart, r, 0, 35).unwrap();

            plot_stairs(r, &chart, stairs[idx], 0, 35).unwrap();

            sum_pdf.clone().collect()
        })
        .collect();

    let all_sum_pdf =
        sum_pdfs.iter().skip(1).fold(sum_pdfs[0].clone(), |acc, v| {
            acc.iter()
                .zip(v)
                .map(|((x1, y1), (_, y2))| (*x1, y1 + y2))
                .collect()
        });

    let r = &right_roots[n_stairs];
    r.titled("b_2", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..0.6_f64);
    chart
        .draw_series(LineSeries::new(all_sum_pdf, BLUE.stroke_width(2)))
        .unwrap();

    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    for stair in stairs {
        plot_stairs(r, &chart, *stair, 0, 35).unwrap();
    }

    let r = &roots[n_stairs];
    r.titled("m_1", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..2.0_f64);
    chart
        .draw_series(LineSeries::new(prev_pdf.clone(), BLUE.stroke_width(2)))
        .unwrap();

    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    let r = &roots[n_stairs + 1];
    r.titled("m_2", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..2.0_f64);
    chart
        .draw_series(LineSeries::new(this_pdf.clone(), BLUE.stroke_width(2)))
        .unwrap();

    for stair in stairs {
        plot_stairs(r, &chart, *stair, 0, 35).unwrap();
    }
    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    Ok(())
}
