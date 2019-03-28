#[test]
fn margin_auto_bottom_and_top_justify_center() {
    let layout = stretch::compute(
        &stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(50f32),
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
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].location.x, 50f32);
    assert_eq!(layout.children[0usize].location.y, 75f32);
    assert_eq!(layout.children[1usize].size.width, 50f32);
    assert_eq!(layout.children[1usize].size.height, 50f32);
    assert_eq!(layout.children[1usize].location.x, 100f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
}
