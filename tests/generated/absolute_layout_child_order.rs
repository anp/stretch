#[test]
fn absolute_layout_child_order() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 110f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 55f32);
    assert_eq!(layout.children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 30f32);
    assert_eq!(layout.children[1usize].size.width, 60f32);
    assert_eq!(layout.children[1usize].size.height, 40f32);
    assert_eq!(layout.children[1usize].location.x, 25f32);
    assert_eq!(layout.children[1usize].location.y, 30f32);
    assert_eq!(layout.children[2usize].size.width, 55f32);
    assert_eq!(layout.children[2usize].size.height, 40f32);
    assert_eq!(layout.children[2usize].location.x, 55f32);
    assert_eq!(layout.children[2usize].location.y, 30f32);
}
