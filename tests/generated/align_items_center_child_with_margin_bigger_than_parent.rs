#[test]
fn align_items_center_child_with_margin_bigger_than_parent() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10f32),
                        end: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 50f32);
    assert_eq!(layout.size.height, 50f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 70f32);
    assert_eq!(layout.children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].location.x, -10f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 10f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
}
