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
    tokyo_xs: &[f64],
    multiplier: f64,
    tokyo_boarding_data: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, tokyo_xs);

    // plot
    let root = &roots[0];
    root.titled("Initial distribution from Tokyo", ("sans-serif", 30))?;
    let mut chart = chart_with_mesh!(root, 0.0..0.06_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    plot_platform_bounds(&chart, root, 0, 35)?;

    for (stair, _, _, _) in tokyo_boarding_data {
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
    let root = &roots[1];
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
    boarding_data: &[(f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    multiplier: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, (root, (stair, far, close, uni))) in
        roots.iter().skip(2).zip(boarding_data).enumerate()
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
    roots: &[DrawingArea<BitMapBackend, Shift>],
    multiplier: f64,
    kanda: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, &xs);

    // plot
    let root = &roots[6];
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
    all_boarding_data: &[Vec<(f64, Vec<f64>, Vec<f64>, Vec<f64>)>],
    filename: &'static str,
    multiplier: f64,
) -> Result<
    (DrawingArea<BitMapBackend<'static>, Shift>, Vec<f64>),
    Box<dyn std::error::Error>,
> {
    let root = BitMapBackend::new(filename, (1024, 1500)).into_drawing_area();
    root.fill(&WHITE)?;

    let tokyo_boarding_data = all_boarding_data[0].clone();
    let kanda_boarding_data = all_boarding_data[1].clone();
    let n_stairs = kanda_boarding_data.len();

    // plus n_stairs for passengers boarding from kanda
    // plus one for passengers already in the train
    // plus one for passengers alighting in kanda
    // plus one for combination of all above
    let roots = root.split_evenly((n_stairs + 3, 1));

    // already in the train
    // as tokyo is the first station, no need to remove passengers that alighted
    // in tokyo. but for other stations after kanda, all preceeding stations
    // need to have their combined net distribution calculated first
    let tokyo_xs = sum_boarding_types(&tokyo_boarding_data);
    plot_initial(&roots, &tokyo_xs, multiplier, &tokyo_boarding_data)?;

    // alighting
    let alight_xs: Vec<_> = tokyo_xs
        .choose_multiple(
            &mut rand::thread_rng(),
            n_passengers_alighting.try_into().unwrap(),
        )
        .collect();

    let mut remaining_xs = tokyo_xs.iter().filter(|x| alight_xs.contains(x));

    plot_alighting(&roots, &mut alight_xs.iter().cloned(), &mut remaining_xs)?;

    // boarding
    plot_boarding(&roots, &kanda_boarding_data, multiplier)?;

    // combined
    let boarding_xs = sum_boarding_types(&kanda_boarding_data);
    let all_xs: Vec<_> = remaining_xs.cloned().chain(boarding_xs).collect();
    plot_combined(all_xs.clone(), &roots, multiplier, &kanda_boarding_data)?;

    Ok((root.clone(), all_xs))
}
