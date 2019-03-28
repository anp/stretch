#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use crate::geometry::{Rect, Size};
use crate::number::Number;
use crate::result::Result;

use decorum::R32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> AlignItems {
        AlignItems::Stretch
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> AlignSelf {
        AlignSelf::Auto
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Inherit
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Display {
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Flex
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

impl FlexDirection {
    pub(crate) fn is_row(self) -> bool {
        self == FlexDirection::Row || self == FlexDirection::RowReverse
    }

    pub(crate) fn is_column(self) -> bool {
        self == FlexDirection::Column || self == FlexDirection::ColumnReverse
    }

    pub(crate) fn is_reverse(self) -> bool {
        self == FlexDirection::RowReverse || self == FlexDirection::ColumnReverse
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> JustifyContent {
        JustifyContent::FlexStart
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Overflow {
        Overflow::Visible
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for PositionType {
    fn default() -> PositionType {
        PositionType::Relative
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> FlexWrap {
        FlexWrap::NoWrap
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(R32),
    Percent(R32),
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Undefined
    }
}

impl Dimension {
    pub(crate) fn resolve(self, parent_width: Number) -> Number {
        match self {
            Dimension::Points(points) => Number::Defined(points),
            Dimension::Percent(percent) => parent_width * percent,
            _ => Number::Undefined,
        }
    }

    pub(crate) fn is_defined(self) -> bool {
        match self {
            Dimension::Points(_) => true,
            Dimension::Percent(_) => true,
            _ => false,
        }
    }
}

// FIXME(anp): reconcile this before PR!
// type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>> + Send + Sync>;
type MeasureFunc = &'static fn(Size<Number>) -> Result<Size<R32>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node {
    pub display: Display,

    pub position_type: PositionType,
    pub direction: Direction,
    pub flex_direction: FlexDirection,

    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,

    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,

    pub justify_content: JustifyContent,

    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,

    pub flex_grow: R32,
    pub flex_shrink: R32,
    pub flex_basis: Dimension,

    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,

    pub aspect_ratio: Number,
    pub measure: Option<MeasureFunc>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            display: Default::default(),

            position_type: Default::default(),
            direction: Default::default(),
            flex_direction: Default::default(),

            flex_wrap: Default::default(),
            overflow: Default::default(),

            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),

            justify_content: Default::default(),

            position: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),

            flex_grow: 0.0.into(),
            flex_shrink: 1.0.into(),
            flex_basis: Dimension::Auto,

            size: Default::default(),
            min_size: Default::default(),
            max_size: Default::default(),

            aspect_ratio: Default::default(),
            measure: None,
        }
    }
}

impl Node {
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.height,
        }
    }

    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.height,
        }
    }

    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.start,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.top,
        }
    }

    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.end,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.bottom,
        }
    }

    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.size.width,
        }
    }

    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.width,
        }
    }

    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.width,
        }
    }

    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.top,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.start,
        }
    }

    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.end,
        }
    }

    pub(crate) fn align_self(&self, parent: &Node) -> AlignSelf {
        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
        }
    }
}
