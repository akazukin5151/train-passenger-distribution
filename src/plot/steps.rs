use crate::make_cumulative;
use crate::plot::utils::*;
use crate::types::*;
use crate::COLORS;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Uniform;

fn sum_boarding_types<T>(
    boarding: &Vec<(T, Vec<f64>, Vec<f64>, Vec<f64>)>,
) -> Vec<f64> {
    boarding.iter().fold(vec![], |acc, (_, far, close, uni)| {
        let mut xs = vec![];
        xs.extend(acc);
        xs.extend(far);
        xs.extend(close);
        xs.extend(uni);
        xs
    })
}

fn plot_initial<T>(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    tokyo_train_passenger: &PassengerLocations,
    multiplier: f64,
    tokyo: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    let i = 0;
    let root = &roots[i];
    root.titled("Initial distribution from Tokyo", ("sans-serif", 30))?;
    let mut chart = basic_chart!(root)
        .margin_top(30_i32)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..0.06_f64)?;

    chart
        .configure_mesh()
        .axis_desc_style(("sans-serif", 20_i32).into_text_style(root))
        .light_line_style(&WHITE)
        .draw()?;

    let kde = make_kde(multiplier, tokyo_train_passenger);
    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    plot_platform_bounds!(chart, root, 0);

    for (stair, _, _, _) in tokyo {
        let drawing_area = chart.plotting_area();
        let mapped = drawing_area.map_coordinate(&(*stair, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1)],
            lighter_stroke(),
        );
        root.draw(&p)?;
    }
    Ok(())
}

fn plot_alighting(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    n_passengers_alighting: i64,
    tokyo_train_passenger: &PassengerLocations,
) -> Result<(), Box<dyn std::error::Error>> {
    let i = 1;
    let root = &roots[i];
    root.titled("Passengers alighting at Kanda", ("sans-serif", 30))?;
    let mut chart = basic_chart!(root)
        .margin_top(30_i32)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..1.0_f64)?;

    chart
        .configure_mesh()
        .axis_desc_style(("sans-serif", 20_i32).into_text_style(root))
        .light_line_style(&WHITE)
        .draw()?;

    let alight_xs: Vec<_> = tokyo_train_passenger
        .passenger_locations
        .choose_multiple(
            &mut rand::thread_rng(),
            n_passengers_alighting.try_into().unwrap(),
        )
        .collect();
    let uniform = Uniform::new(0.0, 1.0_f64);
    let alight_ys = rand::thread_rng().sample_iter(uniform);

    chart
        .draw_series(
            alight_xs
                .iter()
                .zip(alight_ys)
                .map(|(x, y)| Circle::new((**x, y), 2_i32, RED.filled()))
                .choose_multiple(&mut rand::thread_rng(), 200),
        )?
        .label("Alighting")
        .legend(move |(x, y)| {
            Rectangle::new([(x, y - 6), (x + 12, y + 6)], RED.filled())
        });

    let remaining_xs = tokyo_train_passenger
        .passenger_locations
        .iter()
        .filter(|x| alight_xs.contains(x));

    let remaining_ys = rand::thread_rng().sample_iter(uniform);

    chart
        .draw_series(
            remaining_xs
                .zip(remaining_ys)
                .map(|(x, y)| Circle::new((*x, y), 2_i32, GRAY.filled()))
                .choose_multiple(&mut rand::thread_rng(), 200),
        )?
        .label("Remaining")
        .legend(move |(x, y)| {
            Rectangle::new([(x, y - 6), (x + 12, y + 6)], GRAY.filled())
        });

    plot_platform_bounds!(chart, root, 30);
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(&BLACK.mix(0.5))
        .legend_area_size(22_i32)
        .label_font(("sans-serif", 20_i32))
        .draw()?;
    Ok(())
}

fn plot_boarding(
    roots: &[DrawingArea<BitMapBackend, Shift>],
    kanda: &Vec<(f64, Vec<f64>, Vec<f64>, Vec<f64>)>,
    multiplier: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, (root, (stair, far, close, uni))) in
        roots.iter().skip(2).zip(kanda.clone()).enumerate()
    {
        root.titled(
            &format!("Passengers boarding at Kanda stair #{}", i + 1),
            ("sans-serif", 30),
        )?;
        let mut chart = basic_chart!(root)
            .margin_top(30_i32)
            .build_cartesian_2d(-10.0..110.0_f64, 0.0..0.15_f64)?;

        chart
            .configure_mesh()
            .axis_desc_style(("sans-serif", 20_i32).into_text_style(root))
            .light_line_style(&WHITE)
            .draw()?;

        let labels = ["beta far", "beta close", "uniform"];
        for ((xs, label), color) in
            [far, close, uni].iter().zip(labels).zip(COLORS)
        {
            let train_passenger = PassengerLocations {
                passenger_locations: xs.to_vec(),
            };
            let kde = make_kde(multiplier, &train_passenger);
            chart
                .draw_series(LineSeries::new(kde, color.stroke_width(2)))?
                .label(label)
                .legend(move |(x, y)| {
                    Rectangle::new(
                        [(x, y - 6), (x + 12, y + 6)],
                        color.filled(),
                    )
                });
        }

        let modifier = 0;
        plot_platform_bounds!(chart, root, modifier);

        let drawing_area = chart.plotting_area();
        let mapped = drawing_area.map_coordinate(&(stair, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
            lighter_stroke(),
        );
        root.draw(&p)?;

        if i == 0 {
            chart
                .configure_series_labels()
                .position(SeriesLabelPosition::UpperRight)
                .background_style(WHITE.filled())
                .border_style(&BLACK.mix(0.5))
                .legend_area_size(22_i32)
                .label_font(("sans-serif", 20_i32))
                .draw()?;
        }
    }
    Ok(())
}

fn plot_combined<T>(
    n_stairs: usize,
    roots: &[DrawingArea<BitMapBackend, Shift>],
    n_passengers_alighting: i64,
    tokyo_train_passenger: &PassengerLocations,
    kanda_combined: Vec<f64>,
    multiplier: f64,
    kanda: &[(f64, T, T, T)],
) -> Result<(), Box<dyn std::error::Error>> {
    let i = n_stairs + 2;
    let root = &roots[i];
    root.titled("Net distribution after Kanda", ("sans-serif", 30))?;
    let mut chart = basic_chart!(root)
        .margin_top(30_i32)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..0.06_f64)?;

    chart
        .configure_mesh()
        .x_desc("xpos")
        .axis_desc_style(("sans-serif", 20_i32).into_text_style(root))
        .light_line_style(&WHITE)
        .draw()?;

    let xs: Vec<_> = tokyo_train_passenger
        .passenger_locations
        .choose_multiple(
            &mut rand::thread_rng(),
            (tokyo_train_passenger.passenger_locations.len() as i64
                - n_passengers_alighting)
                .try_into()
                .unwrap(),
        )
        .cloned()
        .chain(kanda_combined)
        .collect();
    let tp = PassengerLocations {
        passenger_locations: xs,
    };
    let kde = make_kde(multiplier, &tp);
    chart.draw_series(LineSeries::new(kde, BLUE.stroke_width(2)))?;

    let modifier = 0;
    plot_platform_bounds!(chart, root, modifier);

    for (stair, _, _, _) in kanda {
        let drawing_area = chart.plotting_area();
        let mapped = drawing_area.map_coordinate(&(*stair, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
            lighter_stroke(),
        );
        root.draw(&p)?;
    }
    Ok(())
}

pub fn plot_step_by_step(
    stations: &Vec<&str>,
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
    let tokyo_train_passenger = PassengerLocations {
        passenger_locations: sum_boarding_types(&tokyo),
    };
    plot_initial(&roots, &tokyo_train_passenger, multiplier, &tokyo)?;

    // alighting
    // note that this is not recursive, so for the 3rd station,
    // need to call for 2nd station first
    let n_passengers_alighting = make_cumulative(1, stations.to_vec());
    plot_alighting(&roots, n_passengers_alighting, &tokyo_train_passenger)?;

    // boarding
    plot_boarding(&roots, &kanda, multiplier)?;

    // combined
    let kanda_combined = sum_boarding_types(&kanda);
    plot_combined(
        n_stairs,
        &roots,
        n_passengers_alighting,
        &tokyo_train_passenger,
        kanda_combined,
        multiplier,
        &kanda,
    )?;

    Ok(root.clone())
}
