#[test]
fn absolute_layout_align_items_and_justify_content_center_and_right_position() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 110f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 60f32);
    assert_eq!(layout.children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].location.x, 45f32);
    assert_eq!(layout.children[0usize].location.y, 30f32);
}
