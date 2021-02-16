use crate::reports::{MonthlyResult, TotalResult};

use chrono::Month;
use num_traits::FromPrimitive;
use plotters::prelude::*;

pub fn create_plot(
    monthly: Vec<MonthlyResult>,
    total: TotalResult,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("test.svg", (1240, 1028)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 20, 50, 150);

    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Loan Payment Progress", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Set the ranges for the X and Y axis
        // X should be the number of total terms
        // Y should be the total payment
        .build_cartesian_2d(0..total.planned_terms, 0f64..total.total_cost)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(20)
        .y_labels(10)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // Construct the (X, Y) series data for monthly current_loan
    let remaining: Vec<(i32, f64)> = monthly
        .iter()
        .enumerate()
        .map(|(term, m)| (term as i32, m.current_loan))
        .collect();

    // Construct the (X, Y) series of total cost
    let mut sum: f64 = 0.0;
    let cost: Vec<(i32, f64)> = monthly
        .iter()
        .enumerate()
        .map(|(term, x)| {
            sum += x.payed_back;
            (term as i32, sum.clone())
        })
        .collect();

    // Draw a line between each point
    chart.draw_series(LineSeries::new(remaining.clone(), &RED))?;

    chart.draw_series(LineSeries::new(cost.clone(), &BLACK))?;

    // Draw the text of the last element
    let last_point: (i32, f64) = *cost.last().unwrap();
    chart.draw_series(PointSeries::of_element(
        vec![*remaining.first().unwrap(), last_point],
        5,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:.0?}", c.1), (10, 0), ("sans-serif", 30).into_font());
        },
    ))?;

    // Month/Year of last payment
    let last = monthly.last().unwrap();
    let text = format!(
        "{} {}",
        Month::from_u32(last.month).unwrap().name(),
        last.year
    );
    let cords = chart.backend_coord(remaining.last().unwrap());
    let style = TextStyle::from(("sans-serif", 30).into_font()).color(&RED);
    root.draw_text(&text, &style, (cords.0 - 60, cords.1 - 40))?;

    Ok(())
}
