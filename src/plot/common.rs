// This is a macro to avoid recursive CT (CoordType) bounds requiring Deref
macro_rules! abstract_plot {
    (
        $out_file: expr,
        $y_range: expr,
        $all_station_stairs:ident,
        $make_data:expr
    ) => {{
        let root =
            BitMapBackend::new($out_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let n_stations = $all_station_stairs.len();
        let roots = root.split_evenly((n_stations, 1));
        for (i, root) in roots.iter().enumerate() {
            let mut chart = basic_chart!(&root)
                .build_cartesian_2d::<Range<f64>, Range<f64>>(
                    -10.0..110.0,
                    $y_range,
                )?;

            let mut mesh = chart.configure_mesh();
            let mesh = mesh
                .y_desc(&$all_station_stairs[i].station_name)
                .axis_desc_style(
                    ("Hiragino Sans GB W3", 20_i32).into_text_style(root),
                )
                .light_line_style(&WHITE);
            if i == n_stations - 1 {
                mesh.x_desc("xpos").draw()?;
            } else {
                mesh.draw()?;
            }

            $make_data(i, &mut chart);

            let modifier = 190 * i as i32;
            plot_platform_bounds(&chart, root, modifier, 0)?;

            let drawing_area = chart.plotting_area();
            let stair_locations = &$all_station_stairs[i].stair_locations;
            for stair_location in stair_locations {
                let mapped =
                    drawing_area.map_coordinate(&(*stair_location as f64, 0.0));
                let p: PathElement<(i32, i32)> = PathElement::new(
                    [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
                    crate::plot::colors::lighter_stroke(),
                );
                root.draw(&p)?;
            }
        }
        Ok(root)
    }};
}
