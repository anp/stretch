#[test]
fn flex_basis_smaller_than_main_dimen_column() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100f32), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(10f32),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 50f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
}
