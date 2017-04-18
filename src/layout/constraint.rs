use layout::{LayoutRef, LayoutVars};
use cassowary::{Variable, Constraint};
use cassowary::WeightedRelation::*;
use cassowary::strength::*;
use util::{Scalar, Point, Dimensions};

pub fn match_layout<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    let widget = widget.layout_ref();
    WidgetConstraint::MatchLayout(widget.clone()).builder(REQUIRED)
}
pub fn match_width<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    let widget = widget.layout_ref();
    WidgetConstraint::MatchWidth(widget.left, widget.right).builder(REQUIRED)
}
pub fn match_height<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    let widget = widget.layout_ref();
    WidgetConstraint::MatchWidth(widget.top, widget.bottom).builder(REQUIRED)
}
pub fn width(width: Scalar) -> WidgetConstraintBuilder {
    WidgetConstraint::Width(width).builder(REQUIRED)
}
pub fn height(height: Scalar) -> WidgetConstraintBuilder {
    WidgetConstraint::Height(height).builder(REQUIRED)
}
pub fn min_width(width: Scalar) -> WidgetConstraintBuilder {
    WidgetConstraint::MinWidth(width).builder(REQUIRED)
}
pub fn min_height(height: Scalar) -> WidgetConstraintBuilder {
    WidgetConstraint::MinHeight(height).builder(REQUIRED)
}
pub fn dimensions(dimensions: Dimensions) -> WidgetConstraintBuilder {
    WidgetConstraint::Dimensions(dimensions).builder(REQUIRED)
}
pub fn min_dimensions(dimensions: Dimensions) -> WidgetConstraintBuilder {
    WidgetConstraint::MinDimensions(dimensions).builder(REQUIRED)
}
pub fn shrink() -> WidgetConstraintBuilder {
    WidgetConstraint::Shrink.builder(WEAK)
}
pub fn shrink_horizontal() -> WidgetConstraintBuilder {
    WidgetConstraint::ShrinkHorizontal.builder(WEAK)
}
pub fn shrink_vertical() -> WidgetConstraintBuilder {
    WidgetConstraint::ShrinkVertical.builder(WEAK)
}
pub fn top_left(point: Point) -> WidgetConstraintBuilder {
    WidgetConstraint::TopLeft(point).builder(REQUIRED)
}
pub fn center<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    WidgetConstraint::Center(widget.layout_ref().clone()).builder(REQUIRED)
}
pub fn center_horizontal<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    let widget = widget.layout_ref();
    WidgetConstraint::CenterHorizontal(widget.left, widget.right).builder(REQUIRED)
}
pub fn center_vertical<T: LayoutRef>(widget: &T) -> WidgetConstraintBuilder {
    let widget = widget.layout_ref();
    WidgetConstraint::CenterVertical(widget.top, widget.bottom).builder(REQUIRED)
}

pub fn align_top<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::AlignTop(widget.top).builder(REQUIRED)
}
pub fn align_bottom<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::AlignBottom(widget.bottom).builder(REQUIRED)
}
pub fn align_left<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::AlignLeft(widget.left).builder(REQUIRED)
}
pub fn align_right<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::AlignRight(widget.right).builder(REQUIRED)
}

pub fn above<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::Above(widget.top).builder(REQUIRED)
}
pub fn below<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::Below(widget.bottom).builder(REQUIRED)
}
pub fn to_left_of<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::ToLeftOf(widget.left).builder(REQUIRED)
}
pub fn to_right_of<T: LayoutRef>(widget: &T) -> PaddableConstraintBuilder {
    let widget = widget.layout_ref();
    PaddableConstraint::ToRightOf(widget.right).builder(REQUIRED)
}

pub fn bound_left<T: LayoutRef>(outer: &T) -> PaddableConstraintBuilder {
    let outer = outer.layout_ref();
    PaddableConstraint::BoundLeft(outer.left).builder(REQUIRED)
}
pub fn bound_top<T: LayoutRef>(outer: &T) -> PaddableConstraintBuilder {
    let outer = outer.layout_ref();
    PaddableConstraint::BoundTop(outer.top).builder(REQUIRED)
}
pub fn bound_right<T: LayoutRef>(outer: &T) -> PaddableConstraintBuilder {
    let outer = outer.layout_ref();
    PaddableConstraint::BoundRight(outer.right).builder(REQUIRED)
}
pub fn bound_bottom<T: LayoutRef>(outer: &T) -> PaddableConstraintBuilder {
    let outer = outer.layout_ref();
    PaddableConstraint::BoundBottom(outer.bottom).builder(REQUIRED)
}

pub fn bound_by<T: LayoutRef>(outer: &T) -> PaddableConstraintBuilder {
    let outer = outer.layout_ref();
    PaddableConstraint::BoundBy(outer.clone()).builder(REQUIRED)
}

pub enum WidgetConstraint {
    MatchLayout(LayoutVars),
    MatchWidth(Variable, Variable),
    MatchHeight(Variable, Variable),
    Width(Scalar),
    Height(Scalar),
    MinWidth(Scalar),
    MinHeight(Scalar),
    Dimensions(Dimensions),
    MinDimensions(Dimensions),
    Shrink,
    ShrinkHorizontal,
    ShrinkVertical,
    TopLeft(Point),
    Center(LayoutVars),
    CenterHorizontal(Variable, Variable),
    CenterVertical(Variable, Variable),
}
pub enum PaddableConstraint {
    AlignTop(Variable),
    AlignBottom(Variable),
    AlignLeft(Variable),
    AlignRight(Variable),
    Above(Variable),
    Below(Variable),
    ToLeftOf(Variable),
    ToRightOf(Variable),
    BoundLeft(Variable),
    BoundTop(Variable),
    BoundRight(Variable),
    BoundBottom(Variable),
    BoundBy(LayoutVars),
}
impl WidgetConstraint {
    pub fn builder(self, default_strength: f64) -> WidgetConstraintBuilder {
        WidgetConstraintBuilder {
            constraint: self,
            strength: default_strength,
        }
    }
}
impl PaddableConstraint {
    pub fn builder(self, default_strength: f64) -> PaddableConstraintBuilder {
        PaddableConstraintBuilder {
            constraint: self,
            strength: default_strength,
            padding: 0.0,
        }
    }
}

pub struct WidgetConstraintBuilder {
    constraint: WidgetConstraint,
    strength: f64,
}
impl WidgetConstraintBuilder {
    pub fn strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
}

pub struct PaddableConstraintBuilder {
    constraint: PaddableConstraint,
    strength: f64,
    padding: f64,
}
impl PaddableConstraintBuilder {
    pub fn strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
    pub fn padding(mut self, padding: Scalar) -> Self {
        self.padding = padding;
        self
    }
}

pub trait ConstraintBuilder {
    fn build<T: LayoutRef>(self, widget: &T) -> Vec<Constraint>;
}

impl ConstraintBuilder for WidgetConstraintBuilder {
    fn build<T: LayoutRef>(self, widget: &T) -> Vec<Constraint> {
        let widget = widget.layout_ref();
        let strength = self.strength;
        match self.constraint {
            WidgetConstraint::MatchLayout(other) => {
                vec![
                    widget.left | EQ(strength) | other.left,
                    widget.right | EQ(strength) | other.right,
                    widget.top | EQ(strength) | other.top,
                    widget.bottom | EQ(strength) | other.bottom,
                ]
            }
            WidgetConstraint::MatchWidth(left, right) => {
                vec![ widget.right - widget.left | EQ(strength) | right - left ]
            }
            WidgetConstraint::MatchHeight(top, bottom) => {
                vec![ widget.bottom - widget.top | EQ(strength) | bottom - top ]
            }
            WidgetConstraint::Width(width) => {
                vec![ widget.right - widget.left | EQ(strength) | width ]
            }
            WidgetConstraint::Height(height) => {
                vec![ widget.bottom - widget.top | EQ(strength) | height ]
            }
            WidgetConstraint::MinWidth(width) => {
                vec![ widget.right - widget.left | GE(strength) | width ]
            }
            WidgetConstraint::MinHeight(height) => {
                vec![ widget.bottom - widget.top | GE(strength) | height ]
            }
            WidgetConstraint::Dimensions(dimensions) => {
                vec![
                    widget.right - widget.left | EQ(strength) | dimensions.width,
                    widget.bottom - widget.top | EQ(strength) | dimensions.height,
                ]
            }
            WidgetConstraint::MinDimensions(dimensions) => {
                vec![
                    widget.right - widget.left | GE(strength) | dimensions.width,
                    widget.bottom - widget.top | GE(strength) | dimensions.height,
                ]
            }
            WidgetConstraint::Shrink => {
                vec![
                    widget.right - widget.left | EQ(strength) | 0.0,
                    widget.bottom - widget.top | EQ(strength) | 0.0,
                ]
            }
            WidgetConstraint::ShrinkHorizontal => {
                vec![ widget.right - widget.left | EQ(strength) | 0.0 ]
            }
            WidgetConstraint::ShrinkVertical => {
                vec![ widget.bottom - widget.top | EQ(strength) | 0.0 ]
            }
            WidgetConstraint::TopLeft(point) => {
                vec![
                    widget.left | EQ(strength) | point.x,
                    widget.top | EQ(strength) | point.y,
                ]
            }
            WidgetConstraint::Center(other) => {
                vec![
                    widget.left - other.left | EQ(REQUIRED) | other.right - widget.right,
                    widget.top - other.top | EQ(REQUIRED) | other.bottom - widget.bottom,
                ]
            }
            WidgetConstraint::CenterHorizontal(left, right) => {
                vec![ widget.left - left | EQ(REQUIRED) | right - widget.right ]
            }
            WidgetConstraint::CenterVertical(top, bottom) => {
                vec![ widget.top - top | EQ(REQUIRED) | bottom - widget.bottom ]
            }
        }
    }
}

impl ConstraintBuilder for PaddableConstraintBuilder {
    fn build<T: LayoutRef>(self, widget: &T) -> Vec<Constraint> {
        let widget = widget.layout_ref();
        let strength = self.strength;
        let padding = self.padding;
        match self.constraint {
            PaddableConstraint::AlignTop(top) => {
                vec![ widget.top - top | EQ(strength) | padding ]
            }
            PaddableConstraint::AlignBottom(bottom) => {
                vec![ bottom - widget.bottom | EQ(strength) | padding ]
            }
            PaddableConstraint::AlignLeft(left) => {
                vec![ widget.left - left | EQ(strength) | padding ]
            }
            PaddableConstraint::AlignRight(right) => {
                vec![ right - widget.right | EQ(strength) | padding ]
            }
            PaddableConstraint::Above(top) => {
                vec![ widget.bottom - top | GE(strength) | padding ]
            }
            PaddableConstraint::Below(bottom) => {
                vec![ widget.top - bottom | GE(strength) | padding ]
            }
            PaddableConstraint::ToLeftOf(left) => {
                vec![ left - widget.right | GE(strength) | padding ]
            }
            PaddableConstraint::ToRightOf(right) => {
                vec![ widget.left - right | GE(strength) | padding ]
            }
            PaddableConstraint::BoundLeft(left) => {
                vec![ widget.left - left | GE(strength) | padding ]
            }
            PaddableConstraint::BoundTop(top) => {
                vec![ widget.top - top | GE(strength) | padding ]
            }
            PaddableConstraint::BoundRight(right) => {
                vec![ right - widget.right | GE(strength) | padding ]
            }
            PaddableConstraint::BoundBottom(bottom) => {
                vec![ bottom - widget.bottom | GE(strength) | padding ]
            }
            PaddableConstraint::BoundBy(other) => {
                vec![
                    widget.left - other.left | GE(strength) | padding,
                    widget.top - other.top | GE(strength) | padding,
                    other.right - widget.right | GE(strength) | padding,
                    other.bottom - widget.bottom | GE(strength) | padding,
                ]
            }
        }
    }
}
