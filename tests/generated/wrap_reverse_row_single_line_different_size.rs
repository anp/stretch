#[test]
fn wrap_reverse_row_single_line_different_size() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(300.0000), ..Default::default() },
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
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
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

    assert_eq!(layout.size.width, 300.0000);
    assert_eq!(layout.size.height, 50.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 30.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 40.0000);

    assert_eq!(layout.children[1].size.width, 30.0000);
    assert_eq!(layout.children[1].size.height, 20.0000);
    assert_eq!(layout.children[1].location.x, 30.0000);
    assert_eq!(layout.children[1].location.y, 30.0000);

    assert_eq!(layout.children[2].size.width, 30.0000);
    assert_eq!(layout.children[2].size.height, 30.0000);
    assert_eq!(layout.children[2].location.x, 60.0000);
    assert_eq!(layout.children[2].location.y, 20.0000);

    assert_eq!(layout.children[3].size.width, 30.0000);
    assert_eq!(layout.children[3].size.height, 40.0000);
    assert_eq!(layout.children[3].location.x, 90.0000);
    assert_eq!(layout.children[3].location.y, 10.0000);

    assert_eq!(layout.children[4].size.width, 30.0000);
    assert_eq!(layout.children[4].size.height, 50.0000);
    assert_eq!(layout.children[4].location.x, 120.0000);
    assert_eq!(layout.children[4].location.y, 0.0000);
}
