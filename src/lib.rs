#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub struct Node<A> {
    ptr: *mut ffi::FlexNode,
    context: A,
    children: Vec<Node<A>>,
}

impl<A> Node<A> {
    pub fn new(children: Vec<Self>, props: Vec<FlexProperty>, context: A) -> Self {
        let mut this = unsafe {
            let ptr = ffi::Flex_newNode();
            for child in &children {
                ffi::Flex_addChild(ptr, child.ptr);
            }
            Self {
                ptr,
                children,
                context,
            }
        };

        for prop in props {
            this.set(prop);
        }
        this
    }

    pub fn context(&self) -> &A {
        &self.context
    }

    pub fn children(&self) -> &[Node<A>] {
        &self.children
    }

    pub fn set(&mut self, property: FlexProperty) {
        unsafe {
            match property {
                FlexProperty::Wrap(wrap) => ffi::Flex_setWrap(self.ptr, wrap.into()),
                FlexProperty::Direction(dir) => ffi::Flex_setDirection(self.ptr, dir.into()),
                FlexProperty::AlignItems(val) => ffi::Flex_setAlignItems(self.ptr, val.into()),
                FlexProperty::AlignSelf(val) => ffi::Flex_setAlignSelf(self.ptr, val.into()),
                FlexProperty::AlignContent(val) => ffi::Flex_setAlignContent(self.ptr, val.into()),
                FlexProperty::JustifyContent(val) => ffi::Flex_setJustifyContent(self.ptr, val.into()),
                FlexProperty::Grow(val) => ffi::Flex_setFlexGrow(self.ptr, val),
                FlexProperty::Shrink(val) => ffi::Flex_setFlexShrink(self.ptr, val),
                FlexProperty::Width(Dimension::Auto) => ffi::Flex_setWidthAuto(self.ptr),
                FlexProperty::Width(Dimension::Point(val)) => ffi::Flex_setWidth(self.ptr, val),
                FlexProperty::Width(Dimension::Percent(val)) => ffi::Flex_setWidthPercent(self.ptr, val),
                FlexProperty::Height(Dimension::Auto) => ffi::Flex_setHeightAuto(self.ptr),
                FlexProperty::Height(Dimension::Point(val)) => ffi::Flex_setHeight(self.ptr, val),
                FlexProperty::Height(Dimension::Percent(val)) => ffi::Flex_setHeightPercent(self.ptr, val),
                FlexProperty::MinWidth(val) => ffi::Flex_setMinWidth(self.ptr, val),
                FlexProperty::MinHeight(val) => ffi::Flex_setMinHeight(self.ptr, val),
                FlexProperty::MaxWidth(val) => ffi::Flex_setMaxWidth(self.ptr, val),
                FlexProperty::MaxHeight(val) => ffi::Flex_setMaxHeight(self.ptr, val),
                FlexProperty::MarginLeft(val) => ffi::Flex_setMarginLeft(self.ptr, val),
                FlexProperty::MarginTop(val) => ffi::Flex_setMarginTop(self.ptr, val),
                FlexProperty::MarginBottom(val) => ffi::Flex_setMarginBottom(self.ptr, val),
                FlexProperty::MarginRight(val) => ffi::Flex_setMarginRight(self.ptr, val),
                FlexProperty::PaddingLeft(val) => ffi::Flex_setPaddingLeft(self.ptr, val),
                FlexProperty::PaddingTop(val) => ffi::Flex_setPaddingTop(self.ptr, val),
                FlexProperty::PaddingBottom(val) => ffi::Flex_setPaddingBottom(self.ptr, val),
                FlexProperty::PaddingRight(val) => ffi::Flex_setPaddingRight(self.ptr, val),
                FlexProperty::Fixed(val) => ffi::Flex_setFixed(self.ptr, val),
            }
        }
    }

    pub fn layout(&self, width: Option<f32>, height: Option<f32>) -> NodeWithLayout<A> {
        unsafe {
            ffi::Flex_layout(
                self.ptr,
                width.unwrap_or(f32::NAN),
                height.unwrap_or(f32::NAN),
                1f32,
            )
        };
        NodeWithLayout { node: self }
    }
}

impl<A> Drop for Node<A> {
    fn drop(&mut self) {
        unsafe { ffi::Flex_freeNode(self.ptr) };
    }
}

#[derive(Debug)]
pub struct NodeWithLayout<'a, A> {
    node: &'a Node<A>,
}

impl<'a, A> NodeWithLayout<'a, A> {
    pub fn inner(&self) -> &Node<A> {
        &self.node
    }

    pub fn children(&self) -> impl Iterator<Item = NodeWithLayout<A>> {
        self.node.children.iter().map(|node| NodeWithLayout { node })
    }

    pub fn width(&self) -> f32 {
        unsafe { ffi::Flex_getResultWidth(self.node.ptr) }
    }

    pub fn height(&self) -> f32 {
        unsafe { ffi::Flex_getResultHeight(self.node.ptr) }
    }

    pub fn left(&self) -> f32 {
        unsafe { ffi::Flex_getResultLeft(self.node.ptr) }
    }

    pub fn top(&self) -> f32 {
        unsafe { ffi::Flex_getResultTop(self.node.ptr) }
    }

    pub fn margin_left(&self) -> f32 {
        unsafe { ffi::Flex_getResultMarginLeft(self.node.ptr) }
    }

    pub fn margin_right(&self) -> f32 {
        unsafe { ffi::Flex_getResultMarginRight(self.node.ptr) }
    }

    pub fn margin_top(&self) -> f32 {
        unsafe { ffi::Flex_getResultMarginTop(self.node.ptr) }
    }

    pub fn margin_bottom(&self) -> f32 {
        unsafe { ffi::Flex_getResultMarginBottom(self.node.ptr) }
    }

    pub fn padding_left(&self) -> f32 {
        unsafe { ffi::Flex_getResultPaddingLeft(self.node.ptr) }
    }

    pub fn padding_right(&self) -> f32 {
        unsafe { ffi::Flex_getResultPaddingRight(self.node.ptr) }
    }

    pub fn padding_top(&self) -> f32 {
        unsafe { ffi::Flex_getResultPaddingTop(self.node.ptr) }
    }

    pub fn padding_bottom(&self) -> f32 {
        unsafe { ffi::Flex_getResultPaddingBottom(self.node.ptr) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl From<FlexDirection> for i32 {
    fn from(dir: FlexDirection) -> Self {
        match dir {
            FlexDirection::Row => ffi::FlexDirection_FlexHorizontal,
            FlexDirection::Column => ffi::FlexDirection_FlexVertical,
            FlexDirection::RowReverse => ffi::FlexDirection_FlexHorizontalReverse,
            FlexDirection::ColumnReverse => ffi::FlexDirection_FlexVerticalReverse,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl From<FlexWrap> for i32 {
    fn from(wrap: FlexWrap) -> Self {
        match wrap {
            FlexWrap::NoWrap => ffi::FlexWrapMode_FlexNoWrap,
            FlexWrap::Wrap => ffi::FlexWrapMode_FlexWrap,
            FlexWrap::WrapReverse => ffi::FlexWrapMode_FlexWrapReverse,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexAlign {
    Inherit,
    Stretch,
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    Baseline,
}

impl From<FlexAlign> for i32 {
    fn from(align: FlexAlign) -> Self {
        match align {
            FlexAlign::Inherit => ffi::FlexAlign_FlexInherit,
            FlexAlign::Stretch => ffi::FlexAlign_FlexStretch,
            FlexAlign::Start => ffi::FlexAlign_FlexStart,
            FlexAlign::Center => ffi::FlexAlign_FlexCenter,
            FlexAlign::End => ffi::FlexAlign_FlexEnd,
            FlexAlign::SpaceBetween => ffi::FlexAlign_FlexSpaceBetween,
            FlexAlign::SpaceAround => ffi::FlexAlign_FlexSpaceAround,
            FlexAlign::Baseline => ffi::FlexAlign_FlexBaseline,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Auto,
    Point(f32),
    Percent(f32),
}

#[derive(Debug, Clone)]
pub enum FlexProperty {
    Wrap(FlexWrap),
    Direction(FlexDirection),
    AlignItems(FlexAlign),
    AlignSelf(FlexAlign),
    AlignContent(FlexAlign),
    JustifyContent(FlexAlign),
    Grow(f32),
    Shrink(f32),
    Width(Dimension),
    Height(Dimension),
    MinWidth(f32),
    MinHeight(f32),
    MaxWidth(f32),
    MaxHeight(f32),
    MarginLeft(f32),
    MarginTop(f32),
    MarginBottom(f32),
    MarginRight(f32),
    PaddingLeft(f32),
    PaddingTop(f32),
    PaddingBottom(f32),
    PaddingRight(f32),
    Fixed(bool),
}
