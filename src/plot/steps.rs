use crate::kde::make_kde;
use crate::plot::colors::*;
use crate::plot::utils::*;
use crate::sum_boarding_types;
use crate::COLORS;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;

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

    plot_platform_bounds(&chart, root, 0, 35)?;

    for (stair, _, _, _) in tokyo {
        plot_stairs(root, &chart, *stair, 0, 35)?;
    }
    Ok(())
}

fn plot_alighting(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    mut alight_xs: &mut dyn Iterator<Item = &f64>,
    mut remaining_xs: &mut dyn Iterator<Item = &f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let uniform = Uniform::new(0.0, 1.0_f64);
    let alight_ys = rand::thread_rng().sample_iter(uniform);
    let remaining_ys = rand::thread_rng().sample_iter(uniform);

    // plot
    let i = 1;
    let root = &roots[i];
    root.titled("Passengers alighting at Kanda", ("sans-serif", 30))?;
    let mut chart = chart_with_mesh!(root, 0.0..1.0_f64);

    plot_points(&mut chart, &mut alight_xs, alight_ys, RED, "Alighting")?;
    plot_points(
        &mut chart,
        &mut remaining_xs,
        remaining_ys,
        GRAY,
        "Remaining",
    )?;

    plot_platform_bounds(&chart, root, 210, 30)?;
    add_legend!(&mut chart);
    Ok(())
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

        let modifier = (209 * i + 434) as i32;
        plot_platform_bounds(&chart, root, modifier, 35)?;

        plot_stairs(root, &chart, *stair, modifier, 35)?;

        if i == 0 {
            add_legend!(&mut chart);
        }
    }
    Ok(())
}

fn plot_combined<T>(
    xs: Vec<f64>,
    n_stairs: usize,
    roots: &[DrawingArea<BitMapBackend, Shift>],
    multiplier: f64,
    kanda: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, &xs);

    // plot
    let i = n_stairs + 2;
    let root = &roots[i];
    root.titled("Net distribution after Kanda", ("sans-serif", 30))?;

    let mut chart = chart_with_mesh!(root, 0.0..0.06_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    let modifier = (209 * 4 + 434) as i32;
    plot_platform_bounds(&chart, root, modifier, 35)?;

    for (stair, _, _, _) in kanda {
        plot_stairs(root, &chart, *stair, modifier, 35)?;
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
    // as tokyo is the first station, no need to remove passengers that alighted
    // in tokyo. but for other stations after kanda, all preceeding stations
    // need to have their combined net distribution calculated first
    let tokyo_train_passenger = sum_boarding_types(&tokyo);
    plot_initial(&roots, &tokyo_train_passenger, multiplier, &tokyo)?;

    // alighting
    let alight_xs: Vec<_> = tokyo_train_passenger
        .choose_multiple(
            &mut rand::thread_rng(),
            n_passengers_alighting.try_into().unwrap(),
        )
        .collect();

    let mut remaining_xs = tokyo_train_passenger
        .iter()
        .filter(|x| alight_xs.contains(x));

    plot_alighting(&roots, &mut alight_xs.iter().cloned(), &mut remaining_xs)?;

    // boarding
    plot_boarding(&roots, &kanda, multiplier)?;

    // combined
    let kanda_combined = sum_boarding_types(&kanda);
    plot_combined(
        remaining_xs.cloned().chain(kanda_combined).collect(),
        n_stairs,
        &roots,
        multiplier,
        &kanda,
    )?;

    Ok(root.clone())
}
