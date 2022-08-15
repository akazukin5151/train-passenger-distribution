use crate::plot::utils::*;
use crate::COLORS;
use plotters::chart::SeriesAnno;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Uniform;

fn sum_boarding_types<T>(
    boarding: &[(T, Vec<f64>, Vec<f64>, Vec<f64>)],
) -> Vec<f64> {
    boarding
        .iter()
        .fold(vec![], |mut acc, (_, far, close, uni)| {
            acc.extend(far);
            acc.extend(close);
            acc.extend(uni);
            acc
        })
}

fn plot_stairs(
    root: &DrawingArea<BitMapBackend, Shift>,
    chart: &Chart,
    stair: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let drawing_area = chart.plotting_area();
    let mapped = drawing_area.map_coordinate(&(stair, 0.0));
    let p: PathElement<(i32, i32)> = PathElement::new(
        [(mapped.0, 0), (mapped.0, mapped.1)],
        lighter_stroke(),
    );
    root.draw(&p)?;
    Ok(())
}

trait Ext {
    fn add_legend_icon(&mut self, color: RGBColor);
}

impl Ext for SeriesAnno<'_, BitMapBackend<'_>> {
    fn add_legend_icon(&mut self, color: RGBColor) {
        self.legend(move |(x, y)| {
            Rectangle::new([(x, y - 6), (x + 12, y + 6)], color.filled())
        });
    }
}

fn plot_initial<T>(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    tokyo_train_passenger: &[f64],
    multiplier: f64,
    tokyo: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, tokyo_train_passenger);

    // plot
    let i = 0;
    let root = &roots[i];
    root.titled("Initial distribution from Tokyo", ("sans-serif", 30))?;
    let mut chart = chart_with_mesh!(root, 0.0..0.06_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    plot_platform_bounds(&chart, root, 0)?;

    for (stair, _, _, _) in tokyo {
        plot_stairs(root, &chart, *stair)?;
    }
    Ok(())
}

fn plot_alighting(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    n_passengers_alighting: i64,
    tokyo_train_passenger: &[f64],
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    // data
    let alight_xs: Vec<_> = tokyo_train_passenger
        .choose_multiple(
            &mut rand::thread_rng(),
            n_passengers_alighting.try_into().unwrap(),
        )
        .collect();
    let uniform = Uniform::new(0.0, 1.0_f64);
    let alight_ys = rand::thread_rng().sample_iter(uniform);

    let remaining_xs = tokyo_train_passenger
        .iter()
        .filter(|x| alight_xs.contains(x));

    let remaining_ys = rand::thread_rng().sample_iter(uniform);

    // plot
    let i = 1;
    let root = &roots[i];
    root.titled("Passengers alighting at Kanda", ("sans-serif", 30))?;
    let mut chart = chart_with_mesh!(root, 0.0..1.0_f64);

    chart
        .draw_series(
            alight_xs
                .iter()
                .zip(alight_ys)
                .map(|(x, y)| Circle::new((**x, y), 2_i32, RED.filled()))
                .choose_multiple(&mut rand::thread_rng(), 200),
        )?
        .label("Alighting")
        .add_legend_icon(RED);

    chart
        .draw_series(
            remaining_xs
                .clone()
                .zip(remaining_ys)
                .map(|(x, y)| Circle::new((*x, y), 2_i32, GRAY.filled()))
                .choose_multiple(&mut rand::thread_rng(), 200),
        )?
        .label("Remaining")
        .add_legend_icon(GRAY);

    plot_platform_bounds(&chart, root, 30)?;
    add_legend!(&mut chart);
    Ok(remaining_xs.cloned().collect())
}

fn plot_boarding(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    kanda: &[(f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    multiplier: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, (root, (stair, far, close, uni))) in
        roots.iter().skip(2).zip(kanda).enumerate()
    {
        root.titled(
            &format!("Passengers boarding at Kanda stair #{}", i + 1),
            ("sans-serif", 30),
        )?;
        let mut chart = chart_with_mesh!(root, 0.0..0.15_f64);

        let labels = ["beta far", "beta close", "uniform"];
        for ((xs, label), color) in
            [far, close, uni].iter().zip(labels).zip(COLORS)
        {
            let kde = make_kde(multiplier, xs);
            chart
                .draw_series(LineSeries::new(kde, color.stroke_width(2)))?
                .label(label)
                .add_legend_icon(color);
        }

        let modifier = 0;
        plot_platform_bounds(&chart, root, modifier)?;

        plot_stairs(root, &chart, *stair)?;

        if i == 0 {
            add_legend!(&mut chart);
        }
    }
    Ok(())
}

fn plot_combined<T>(
    remaining_xs: Vec<f64>,
    n_stairs: usize,
    roots: &[DrawingArea<BitMapBackend, Shift>],
    kanda_combined: Vec<f64>,
    multiplier: f64,
    kanda: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let xs: Vec<_> =
        remaining_xs.iter().cloned().chain(kanda_combined).collect();
    let kde = make_kde(multiplier, &xs);

    // plot
    let i = n_stairs + 2;
    let root = &roots[i];
    root.titled("Net distribution after Kanda", ("sans-serif", 30))?;

    let mut chart = chart_with_mesh!(root, 0.0..0.06_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    let modifier = 0;
    plot_platform_bounds(&chart, root, modifier)?;

    for (stair, _, _, _) in kanda {
        plot_stairs(root, &chart, *stair)?;
    }
    Ok(())
}

pub fn plot_step_by_step(
    n_passengers_alighting: i64,
    result: &[Vec<(f64, Vec<f64>, Vec<f64>, Vec<f64>)>],
    filename: &'static str,
    multiplier: f64,
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    let root = BitMapBackend::new(filename, (1024, 1500)).into_drawing_area();
    root.fill(&WHITE)?;

    let tokyo = result[0].clone();
    let kanda = result[1].clone();
    let n_stairs = kanda.len();

    // plus n_stairs for passengers boarding from kanda
    // plus one for passengers already in the train
    // plus one for passengers alighting in kanda
    // plus one for combination of all above
    let roots = root.split_evenly((n_stairs + 3, 1));

    // already in the train
    let tokyo_train_passenger = sum_boarding_types(&tokyo);
    plot_initial(&roots, &tokyo_train_passenger, multiplier, &tokyo)?;

    // alighting
    // note that this is not recursive, so for the 3rd station,
    // need to call for 2nd station first
    let remaining_xs =
        plot_alighting(&roots, n_passengers_alighting, &tokyo_train_passenger)?;

    // boarding
    plot_boarding(&roots, &kanda, multiplier)?;

    // combined
    let kanda_combined = sum_boarding_types(&kanda);
    plot_combined(
        remaining_xs,
        n_stairs,
        &roots,
        kanda_combined,
        multiplier,
        &kanda,
    )?;

    Ok(root.clone())
}
