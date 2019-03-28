#[test]
fn percent_within_flex_grow() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(350.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
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

    assert_eq!(layout.size.width, 350.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 150.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 100.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[1].children[0].size.width, 150.0000);
    assert_eq!(layout.children[1].children[0].size.height, 0.0000);
    assert_eq!(layout.children[1].children[0].location.x, 0.0000);
    assert_eq!(layout.children[1].children[0].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 100.0000);
    assert_eq!(layout.children[2].size.height, 100.0000);
    assert_eq!(layout.children[2].location.x, 250.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);
}
