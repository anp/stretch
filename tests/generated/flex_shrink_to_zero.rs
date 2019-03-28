#[test]
fn flex_shrink_to_zero() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(75f32), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 0f32,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 1f32,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0f32,
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
    assert_eq!(layout.size.width, 75f32);
    assert_eq!(layout.size.height, 50f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 0f32);
    assert_eq!(layout.children[1usize].size.height, 50f32);
    assert_eq!(layout.children[1usize].location.x, 50f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
    assert_eq!(layout.children[2usize].size.width, 50f32);
    assert_eq!(layout.children[2usize].size.height, 50f32);
    assert_eq!(layout.children[2usize].location.x, 50f32);
    assert_eq!(layout.children[2usize].location.y, 0f32);
}
