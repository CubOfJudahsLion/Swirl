//! # Module        :   _Macro_
//! - **Purpose**   :   A single macro that builds layouts
//!                     using shortened notation.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Thu 06/09/2022 10:15 AM
//! - **Modified**  :   ~


#[macro_export]
macro_rules! layoutm {
    (@pad nopad) => {
        crate::Padding::NoPadding  
    };
    (@pad pad($uniform:expr)) => {
        crate::Padding::Uniform($uniform)
    };
    (@pad pad($width:expr, $height:expr)) => {
        crate::Padding::Directional(crate::Dimensions {width: $width, height: $height,})
    };
    (@pad pad($top:expr, $left:expr, $bottom:expr, $right:expr)) => {
        crate::Padding::EachEdge(crate::Rect {top: $top, left: $left, bottom: $bottom, right: $right})
    };

    (@h l)      => { crate::HorizontalAlign::Left };
    (@h left)   => { crate::HorizontalAlign::Left };
    (@h c)      => { crate::HorizontalAlign::Center };
    (@h center) => { crate::HorizontalAlign::Center };
    (@h r)      => { crate::HorizontalAlign::Right };
    (@h right)  => { crate::HorizontalAlign::Right };

    (@v t)      => { crate::VerticalAlign::Top };
    (@v top)    => { crate::VerticalAlign::Top };
    (@v m)      => { crate::VerticalAlign::Middle };
    (@v middle) => { crate::VerticalAlign::Middle };
    (@v b)      => { crate::VerticalAlign::Bottom };
    (@v bottom) => { crate::VerticalAlign::Bottom };

    (@hc min)                   => { crate::WidthConstraints::MinWidth };
    (@hc max)                   => { crate::WidthConstraints::MaxWidth };
    (@hc pix($pixels:expr))     => { crate::WidthConstraints::MinPixelWidth($pixels) };
    (@hc parts($parts:expr))    => { crate::WidthConstraints::HorizontalParts($parts) };

    (@vc min)                   => { crate::HeightConstraints::MinHeight };
    (@vc max)                   => { crate::HeightConstraints::MaxHeight };
    (@vc pix($pixels:expr))     => { crate::HeightConstraints::MinPixelHeight($pixels) };
    (@vc parts($parts:expr))    => { crate::HeightConstraints::VerticalParts($parts) };

    (@name)                 => { None };
    (@name [$name:expr])    => { Some($name) };

    (@inner $layout_type:ident $($rest:tt)+) => {
        &(layoutm!($layout_type $($rest)+)?)
    };

    (single $([$name:expr],)? ($pad:tt, $h:ident, $v:ident, draw($drawable:expr))) => {
        Ok(crate::SingleObject::new_from_drawable(
            layoutm!(@name $([$name:expr])?),
            layoutm!(@pad $pad),
            layoutm!(@h $h),
            layoutm!(@v $v),
            &($drawable)
        ).0)
    };
    (single $([$name:expr],)? ($pad:tt, $h:ident, $v:ident, node($node:expr)))     => {
        Ok(crate::SingleObject::new(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $pad),
            layoutm!(@h $h),
            layoutm!(@v $v),
            ($node).clone() 
        ))
    };

    (superpos $([$name:expr],)? ($pad:tt, $h:ident, $v:ident) { $($layout_type:ident $($trees:tt)+);+ $(;)? })  => {
        Ok(create::Superposition::new(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $pad),
            layoutm!(@h $h),
            layoutm!(@v $v),

        ))
        $(
            .and_then(|layout| layout.with(&(
                layoutm!(@inner $layout_type $($trees)+)
            )))
        )+
    };

    (row $([$name:expr],)? ($pad:tt, $h:ident, $v:ident) { $(($constr:tt, $zorder:expr) $layout_type:ident $($trees:tt)+);+ $(;)? })       => {
        Ok(crate::Row::new(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $pad),
            layoutm!(@h $h),
            layoutm!(@v $v),
        ))
        $(
            .and_then(|layout| layout.with(&(
                layoutm!(@hc $constr),
                $zorder,
                layoutm!(@inner $layout_type $($trees)+)
            )))
        )+
    };

    (col $([$name:expr],)? ($pad:tt, $h:ident, $v:ident) { $(($constr:tt, $zorder:expr) $layout_type:ident $($trees:tt)+);+ $(;)? })       => {
        Ok(crate::Row::new(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $pad),
            layoutm!(@h $h),
            layoutm!(@v $v),
        ))
        $(
            .and_then(|layout| layout.with(&(
                layoutm!(@vc $constr),
                $zorder,
                layoutm!(@inner $layout_type $($trees)+)
            )))
        )+
    };
}
