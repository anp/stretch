#[test]
fn wrapped_column_max_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(700f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100f32),
                        height: stretch::style::Dimension::Points(500f32),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(200f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200f32),
                        height: stretch::style::Dimension::Points(200f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(20f32),
                        end: stretch::style::Dimension::Points(20f32),
                        top: stretch::style::Dimension::Points(20f32),
                        bottom: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100f32),
                        height: stretch::style::Dimension::Points(100f32),
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
    assert_eq!(layout.size.width, 700f32);
    assert_eq!(layout.size.height, 500f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].size.height, 200f32);
    assert_eq!(layout.children[0usize].location.x, 250f32);
    assert_eq!(layout.children[0usize].location.y, 30f32);
    assert_eq!(layout.children[1usize].size.width, 200f32);
    assert_eq!(layout.children[1usize].size.height, 200f32);
    assert_eq!(layout.children[1usize].location.x, 200f32);
    assert_eq!(layout.children[1usize].location.y, 250f32);
    assert_eq!(layout.children[2usize].size.width, 100f32);
    assert_eq!(layout.children[2usize].size.height, 100f32);
    assert_eq!(layout.children[2usize].location.x, 420f32);
    assert_eq!(layout.children[2usize].location.y, 200f32);
}
