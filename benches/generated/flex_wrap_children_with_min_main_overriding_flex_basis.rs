pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_basis: stretch::style::Dimension::Points(50f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                flex_basis: stretch::style::Dimension::Points(50f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
