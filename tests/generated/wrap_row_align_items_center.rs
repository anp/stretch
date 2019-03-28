#[test]
fn wrap_row_align_items_center() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
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

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 60.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 30.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 10.0000);

    assert_eq!(layout.children[1].size.width, 30.0000);
    assert_eq!(layout.children[1].size.height, 20.0000);
    assert_eq!(layout.children[1].location.x, 30.0000);
    assert_eq!(layout.children[1].location.y, 5.0000);

    assert_eq!(layout.children[2].size.width, 30.0000);
    assert_eq!(layout.children[2].size.height, 30.0000);
    assert_eq!(layout.children[2].location.x, 60.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);

    assert_eq!(layout.children[3].size.width, 30.0000);
    assert_eq!(layout.children[3].size.height, 30.0000);
    assert_eq!(layout.children[3].location.x, 0.0000);
    assert_eq!(layout.children[3].location.y, 30.0000);
}
