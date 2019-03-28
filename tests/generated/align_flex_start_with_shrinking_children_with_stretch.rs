#[test]
fn align_flex_start_with_shrinking_children_with_stretch() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexStart,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 500.0000);
    assert_eq!(layout.size.height, 500.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].size.height, 500.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].children[0].size.height, 0.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
}
