#[test]
fn absolute_layout_start_top_end_bottom() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Points(10.0000),
                    top: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 80.0000);
    assert_eq!(layout.children[0].size.height, 80.0000);
    assert_eq!(layout.children[0].location.x, 10.0000);
    assert_eq!(layout.children[0].location.y, 10.0000);
}
