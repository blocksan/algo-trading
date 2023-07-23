let prices = vec![100.0, 110.0, 120.0, 105.0, 90.0, 115.0, 95.0, 110.0, 120.0];

    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_price = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_price = prices.iter().cloned().fold(f64::INFINITY, f64::min);

    let mut chart = ChartBuilder::on(&root)
        .caption("Historical Prices", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..prices.len() as f64, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    // Use a thicker line style
    chart
        .draw_series(LineSeries::new(
            prices.iter().enumerate().map(|(x, y)| (x as f64, *y)),
            &BLUE.mix(0.2), // Use transparency to make it less dominant
        ))?
    .label("Price Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.2)));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    // Support and Resistance Levels
    let supports = vec![4, 6];
    let resistances = vec![2, 5, 8];

    let support_data: Vec<(f64, f64)> = supports
        .iter()
        .map(|&index| (index as f64, prices[index]))
        .collect();

    let resistance_data: Vec<(f64, f64)> = resistances
        .iter()
        .map(|&index| (index as f64, prices[index]))
        .collect();

    chart
        .draw_series(PointSeries::of_element(
            support_data,
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            },
        ))?
        .label("Support")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(PointSeries::of_element(
            resistance_data,
            5,
            ShapeStyle::from(&GREEN).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            },
        ))?
        .label("Resistance")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart.configure_series_labels().draw()?;
    Ok(())