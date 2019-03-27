#[test]
fn percentage_flex_basis_cross() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.25f32),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 400f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 200f32);
    assert_eq!(layout.children[0usize].size.height, 250f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 200f32);
    assert_eq!(layout.children[1usize].size.height, 150f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 250f32);
}
