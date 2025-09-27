//! # Module        :   _Macro_
//! - **Purpose**   :   A single macro that builds layouts
//!                     using shortened notation.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Thu 06/09/2022 10:15 AM
//! - **Modified**  :   Wed 29 Jun 2022 01:15:44 PM CST


#[macro_export]
macro_rules! layoutm {
    (@pad )                                                 => {
        $crate::Padding::NoPadding
    };
    (@pad $uniform:expr)                                    => {
        $crate::Padding::Uniform($uniform)
    };
    (@pad $width:expr, $height:expr)                        => {
        $crate::Padding::Directional($crate::Dimensions {width: $width, height: $height,})
    };
    (@pad $top:expr, $left:expr, $bottom:expr, $right:expr) => {
        $crate::Padding::EachEdge($crate::Rect {
            top: $top, left: $left, bottom: $bottom, right: $right
        })
    };

    (@h )       => { <$crate::HorizontalAlign as std::default::Default>::default() };
    (@h l)      => { $crate::HorizontalAlign::Left };
    (@h left)   => { $crate::HorizontalAlign::Left };
    (@h c)      => { $crate::HorizontalAlign::Center };
    (@h center) => { $crate::HorizontalAlign::Center };
    (@h r)      => { $crate::HorizontalAlign::Right };
    (@h right)  => { $crate::HorizontalAlign::Right };

    (@v )       => { <$crate::VerticalAlign as std::default::Default>::default() };
    (@v t)      => { $crate::VerticalAlign::Top };
    (@v top)    => { $crate::VerticalAlign::Top };
    (@v m)      => { $crate::VerticalAlign::Middle };
    (@v middle) => { $crate::VerticalAlign::Middle };
    (@v b)      => { $crate::VerticalAlign::Bottom };
    (@v bottom) => { $crate::VerticalAlign::Bottom };

    (@hc )                      => { <$crate::WidthConstraints as std::default::Default>::default() };
    (@hc min)                   => { $crate::WidthConstraints::MinWidth };
    (@hc max)                   => { $crate::WidthConstraints::MaxWidth };
    (@hc pix($pixels:expr))     => { $crate::WidthConstraints::MinPixelWidth($pixels) };
    (@hc parts($parts:expr))    => { $crate::WidthConstraints::HorizontalParts($parts) };

    (@vc )                      => { <$crate::HeightConstraints as std::default::Default>::default() };
    (@vc min)                   => { $crate::HeightConstraints::MinHeight };
    (@vc max)                   => { $crate::HeightConstraints::MaxHeight };
    (@vc pix($pixels:expr))     => { $crate::HeightConstraints::MinPixelHeight($pixels) };
    (@vc parts($parts:expr))    => { $crate::HeightConstraints::VerticalParts($parts) };

    (@s )                   => { $crate::LayoutScaling::NoScaling };
    (@s $scale:expr)        => { $crate::LayoutScaling::UniformScaling($scale) };
    (@s $sx:expr, $sy:expr) => { $crate::LayoutScaling::Scaling($sx, $sy) };

    (@name)                 => { None };
    (@name [$name:expr])    => { Some($name) };

    //(@inner $layout_type:ident $($rest:tt)+) => {
    //    (&((layoutm!($layout_type $($rest)+))?))
    //};

    (@inner
        $layout_type:ident $([$sub_name:expr])?
            ($($spec:tt)*)
            $({$($sub_layouts:tt)+})?
    ) => {
        (&((layoutm!($layout_type $([$sub_name])? ($($spec)*) $({$($sub_layouts)+})?))?))
    };

    (
        lay ($lay:expr)
    ) => {
        Ok(($lay).clone())
    };

    (
        single $([$name:expr])?
            ($(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))? draw($drawable:expr))
    ) => {
        Ok($crate::SingleObject::new_from_drawable(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $($($dim),+)?),
            layoutm!(@h $($h)?),
            layoutm!(@v $($v)?),
            layoutm!(@s $($s1 $(,$s2)?)?),
            &($drawable)
        ).0)
    };

    (
        single $([$name:expr])?
            ($(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))? node($node:expr))
    ) => {
        Ok($crate::SingleObject::new(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $($($dim),+)?),
            layoutm!(@h $($h)?),
            layoutm!(@v $($v)?),
            layoutm!(@s $($s1 $(,$s2)?)?),
            ($node).clone()
        ))
    };

    (
        single $([$name:expr])?
            ($(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))? text($text:expr))
    ) => {
        Ok($crate::SingleObject::new_from_text(
            layoutm!(@name $([$name])?),
            layoutm!(@pad $($($dim),+)?),
            layoutm!(@h $($h)?),
            layoutm!(@v $($v)?),
            layoutm!(@s $($s1 $(,$s2)?)?),
            &($text)
        ).0)
    };


    (
        superpos $([$name:expr])?
            ($(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))?)
            {
                $(
                    $layout_type:ident
                        $([$sub_name:expr])?
                        ($($spec:tt)*)
                        $({$($sub_layouts:tt)+})?
                ),+
                $(,)?
            }
    ) => {
        (
            Ok($crate::Superposition::new(
                layoutm!(@name $([$name])?),
                layoutm!(@pad $($($dim),+)?),
                layoutm!(@h $($h)?),
                layoutm!(@v $($v)?),
                layoutm!(@s $($s1 $(,$s2)?)?)
            ))
            $(
                .and_then(|layout| layout.with(
                    layoutm!(@inner
                        $layout_type $([$sub_name])? ($($spec)*) $({$($sub_layouts)+})?
                    )
                ))
            )+
        )
    };

    (
        row $([$name:expr])?
            ($(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))?)
            {
                $(
                    ($constr:ident $(($constr_args:expr))?, $zorder:expr)
                    $layout_type:ident
                        $([$sub_name:expr])?
                        ($($spec:tt)*)
                        $({$($sub_layouts:tt)+})?
                ),+
                $(,)?
            }
    ) => {
        (
            Ok($crate::Row::new(
                layoutm!(@name $([$name])?),
                layoutm!(@pad $($($dim),+)?),
                layoutm!(@h $($h)?),
                layoutm!(@v $($v)?),
                layoutm!(@s $($s1 $(,$s2)?)?)
            ))
            $(
                .and_then(|layout| layout.with(
                    layoutm!(@hc $constr $(($constr_args))?),
                    $zorder,
                    layoutm!(@inner
                        $layout_type $([$sub_name])? ($($spec)*) $({$($sub_layouts)+})?
                    )
                ))
            )+
        )
    };

    (
        col $([$name:expr])?
            ( $(pad($($dim:expr),+))? $(h($h:ident))? $(v($v:ident))? $(s($s1:expr$(,$s2:expr)?))? )
            {
                $(
                    ($constr:ident $(($constr_args:expr))?, $zorder:expr)
                    $layout_type:ident
                        $([$sub_name:expr])?
                        ($($spec:tt)*)
                        $({$($sub_layouts:tt)+})?
                ),+
                $(,)?
            }
    ) => {
        (
            Ok($crate::Column::new(
                layoutm!(@name $([$name])?),
                layoutm!(@pad $($($dim),+)?),
                layoutm!(@h $($h)?),
                layoutm!(@v $($v)?),
                layoutm!(@s $($s1 $(,$s2)?)?)
            ))
            $(
                .and_then(|layout| layout.with(
                    layoutm!(@vc $constr $(($constr_args))?),
                    $zorder,
                    layoutm!(@inner
                        $layout_type $([$sub_name])? ($($spec)*) $({$($sub_layouts)+})?
                    )
                ))
            )+
        )
    };
}
