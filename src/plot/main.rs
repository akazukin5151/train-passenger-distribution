use crate::plot::colors::*;
use crate::plot::utils::*;
use crate::types::*;
use plotters::prelude::*;

pub const COLORS: [RGBColor; 4] = [
    RGBColor(76, 114, 176),
    RGBColor(221, 132, 82),
    RGBColor(85, 168, 104),
    RGBColor(196, 78, 82),
];

pub fn plot_pdfs(
    filename: &str,
    all_station_stairs: &[StationStairs],
    pdfs: Vec<Vec<(f64, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let roots = root.split_evenly((pdfs.len(), 1));

    for ((idx, r), station) in roots.iter().enumerate().zip(all_station_stairs)
    {
        r.titled(&station.station_name, ("Hiragino Sans GB W3", 20_i32))?;
        let mut chart = basic_chart!(r)
            .margin_top(30_i32)
            .build_cartesian_2d(-10.0..110.0_f64, 0.0..2.0_f64)
            .unwrap();

        let mut mesh = chart.configure_mesh();
        let mesh = mesh
            .y_desc("density")
            .axis_desc_style(("sans-serif", 20_i32).into_text_style(r))
            .light_line_style(&WHITE);
        if idx == pdfs.len() - 1 {
            mesh.x_desc("xpos").draw()?;
        } else {
            mesh.draw()?;
        }

        for (i, pdf) in pdfs.iter().enumerate() {
            let color = if i == idx {
                let c: RGBColor = COLORS[i];
                c.stroke_width(2)
            } else {
                GRAY.filled()
            };
            chart.draw_series(LineSeries::new(pdf.clone(), color))?;
        }

        let modifier = (((192 * idx) as f32) - 0.5) as i32;
        plot_platform_bounds(&chart, r, modifier, 30)?;

        for stair in &station.stair_locations {
            plot_stairs(r, &chart, *stair, modifier, 30).unwrap();
        }
    }

    Ok(())
}

pub fn plot_stair_pdfs_sep(
    filename: &str,
    pdfs: Vec<Vec<(f64, (f64, f64, f64))>>,
    stairs: &[f64],
    prev_pdf: &[(f64, f64)],
    this_pdf: &[(f64, f64)],
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
            add_legend!(chart, "sans-serif").unwrap();
        }
    }

    let sum_pdfs: Vec<Vec<(f64, f64)>> = right_roots
        .iter()
        .enumerate()
        .take(n_stairs)
        .map(|(idx, r)| {
            r.titled(
                &format!(
                    "All passengers boarding at Ochanomizu stair #{} (S_{}/{})",
                    idx + 1,
                    idx + 1,
                    n_stairs
                ),
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
    r.titled("All boarders at Ochanomizu (b_2)", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..0.6_f64);
    chart
        .draw_series(LineSeries::new(all_sum_pdf, BLUE.stroke_width(2)))
        .unwrap();

    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    for stair in stairs {
        plot_stairs(r, &chart, *stair, 0, 35).unwrap();
    }

    let r = &roots[n_stairs];
    r.titled("PDF of Kanda (m_1)", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..2.0_f64);
    chart
        .draw_series(LineSeries::new(prev_pdf.to_owned(), BLUE.stroke_width(2)))
        .unwrap();

    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    let r = &roots[n_stairs + 1];
    r.titled("PDF of Ochanomizu (m_2)", ("sans-serif", 30_i32))?;
    let mut chart = chart_with_mesh!(r, 0.0..2.0_f64);
    chart
        .draw_series(LineSeries::new(this_pdf.to_owned(), BLUE.stroke_width(2)))
        .unwrap();

    for stair in stairs {
        plot_stairs(r, &chart, *stair, 0, 35).unwrap();
    }
    plot_platform_bounds(&chart, r, 0, 35).unwrap();

    Ok(())
}

pub fn plot_pdfs_together(
    filename: &str,
    all_station_stairs: &[StationStairs],
    pdfs: Vec<Vec<(f64, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = basic_chart!(&root)
        .margin_top(30_i32)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..2.0_f64)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("xpos")
        .y_desc("density")
        .axis_desc_style(("sans-serif", 20_i32).into_text_style(&root))
        .light_line_style(&WHITE)
        .draw()?;

    for ((pdf, color), station) in
        pdfs.iter().zip(COLORS).zip(all_station_stairs)
    {
        chart
            .draw_series(LineSeries::new(pdf.clone(), color.stroke_width(3)))?
            .label(&station.station_name)
            .add_legend_icon(color);
    }
    plot_platform_bounds(&chart, &root, 0, 35)?;
    add_legend!(chart, "Hiragino Sans GB W3")?;

    Ok(())
}
