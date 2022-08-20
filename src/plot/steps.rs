use crate::kde::make_kde;
use crate::plot::colors::*;
use crate::plot::utils::*;
use crate::types::*;
use crate::Accumulator;
use crate::COLORS;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;

fn plot_initial(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    tokyo_xs: &[f64],
    multiplier: f64,
    tokyo_boarding_data: &Vec<BoardingData>,
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, tokyo_xs);

    // plot
    let root = &roots[0];
    root.titled("Initial distribution from Kanda", ("sans-serif", 30))?;
    let mut chart = chart_with_mesh!(root, 0.0..0.1_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    plot_platform_bounds(&chart, root, 0, 35)?;

    for bd in tokyo_boarding_data {
        plot_stairs(root, &chart, bd.stair_location, 0, 35)?;
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
    root.titled("Passengers alighting at Ochanomizu", ("sans-serif", 30))?;
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
    add_legend!(&mut chart, "sans-serif");
    Ok(())
}

fn plot_boarding(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    boarding_data: &Vec<BoardingData>,
    multiplier: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, (root, bd)) in roots.iter().skip(2).zip(boarding_data).enumerate() {
        root.titled(
            &format!("Passengers boarding at Ochanomizu stair #{}", i + 1),
            ("sans-serif", 30),
        )?;
        let mut chart = chart_with_mesh!(root, 0.0..0.25_f64);

        let labels = ["beta far", "beta close", "uniform"];
        for ((xs, label), color) in [&bd.beta_far, &bd.beta_close, &bd.uniform]
            .iter()
            .zip(labels)
            .zip(COLORS)
        {
            let kde = make_kde(multiplier, xs);
            chart
                .draw_series(LineSeries::new(kde, color.stroke_width(2)))?
                .label(label)
                .add_legend_icon(color);
        }

        let modifier = (209 * i + 434) as i32;
        plot_platform_bounds(&chart, root, modifier, 35)?;

        plot_stairs(root, &chart, bd.stair_location, modifier, 35)?;

        if i == 0 {
            add_legend!(&mut chart, "sans-serif");
        }
    }
    Ok(())
}

fn plot_all_boarding(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    boarding_data: &Vec<BoardingData>,
    multiplier: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, (root, bd)) in roots.iter().skip(2).zip(boarding_data).enumerate() {
        let mut acc: Vec<f64> = vec![];
        acc.extend(&bd.beta_far);
        acc.extend(&bd.beta_close);
        acc.extend(&bd.uniform);

        root.titled(
            &format!("All passengers boarding at Ochanomizu stair #{}", i + 1),
            ("sans-serif", 30),
        )?;
        let mut chart = chart_with_mesh!(root, 0.0..0.25_f64);

        let kde = make_kde(multiplier, &acc);
        chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

        let modifier = (209 * i + 434) as i32;
        plot_platform_bounds(&chart, root, modifier, 35)?;

        plot_stairs(root, &chart, bd.stair_location, modifier, 35)?;
    }
    Ok(())
}

fn plot_combined(
    title: &str,
    xs: &[f64],
    roots: &[DrawingArea<BitMapBackend, Shift>],
    multiplier: f64,
    kanda: &Vec<BoardingData>,
) -> Result<(), Box<dyn std::error::Error>> {
    // data
    let kde = make_kde(multiplier, xs);

    // plot
    let root = &roots[6];
    root.titled(title, ("sans-serif", 30))?;

    let mut chart = chart_with_mesh!(root, 0.0..0.1_f64);

    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    let modifier = (209 * 4 + 434) as i32;
    plot_platform_bounds(&chart, root, modifier, 35)?;

    for bd in kanda {
        plot_stairs(root, &chart, bd.stair_location, modifier, 35)?;
    }
    Ok(())
}

pub fn plot_step_by_step(
    all_steps: &Vec<Accumulator>,
    filename: &'static str,
    multiplier: f64,
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    // data
    let initial_step = &all_steps[1];
    let initial_boarding_data = &initial_step.boarding_data;
    let initial_xs = &initial_step.all_xs;

    let this_step = &all_steps[2];
    let this_boarding_data = &this_step.boarding_data;
    let n_passengers_alighting = &this_step.n_passengers_alighting;

    // alighting
    let alight_xs: Vec<_> = initial_xs
        .choose_multiple(
            &mut rand::thread_rng(),
            (*n_passengers_alighting).try_into().unwrap(),
        )
        .collect();

    let remaining_xs = &this_step.remaining_xs;

    // combined
    let all_xs: &Vec<_> = &this_step.all_xs;

    // plot
    let root =
        BitMapBackend::new(filename, (1024 * 2, 1500)).into_drawing_area();
    root.fill(&WHITE)?;

    // plus n_stairs for passengers boarding from kanda
    // plus one for passengers already in the train
    // plus one for passengers alighting in kanda
    // plus one for combination of all above
    let n_stairs = this_boarding_data.len();
    let (left, right) = root.split_horizontally(1024);
    let right_roots = right.split_evenly((n_stairs + 3, 1));
    let roots = left.split_evenly((n_stairs + 3, 1));

    plot_initial(&roots, initial_xs, multiplier, initial_boarding_data)?;

    plot_alighting(
        &roots,
        &mut alight_xs.iter().cloned(),
        &mut remaining_xs.iter(),
    )?;

    plot_boarding(&roots, this_boarding_data, multiplier)?;

    plot_all_boarding(&right_roots, this_boarding_data, multiplier)?;

    let mut bs = vec![];
    for bd in this_boarding_data {
        let mut acc: Vec<f64> = vec![];
        acc.extend(&bd.beta_far);
        acc.extend(&bd.beta_close);
        acc.extend(&bd.uniform);
        bs.extend(acc);
    }
    plot_combined(
        "All boarders at Ochanomizu",
        &bs,
        &right_roots,
        multiplier,
        this_boarding_data,
    )?;

    plot_combined(
        "Net distribution after Ochanomizu",
        all_xs,
        &roots,
        multiplier,
        this_boarding_data,
    )?;

    Ok(root.clone())
}
