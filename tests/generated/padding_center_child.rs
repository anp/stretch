#[test]
fn padding_center_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10f32),
                    height: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(20f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 10f32);
    assert_eq!(layout.children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].location.x, 40f32);
    assert_eq!(layout.children[0usize].location.y, 40f32);
}
