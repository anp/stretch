#[test]
fn justify_content_column_min_height_and_margin_top() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 20.0000);
    assert_eq!(layout.size.height, 50.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 20.0000);
    assert_eq!(layout.children[0].size.height, 20.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 20.0000);
}
