//! Module  :   Font
//! Purpose :   Representation of font resources
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 05/11/2021 01:46 PM
//! Modified:   --


use crate::make_resource_set;


//  Font directory.
const DIR: &str = "fonts";


//  The single set of fonts.
make_resource_set! {
    Set ::=
        TonicLiver      => "LHFTonicLIVER",
        TekoSemibold    => "teko-semibold";
    << "{}/{}.woff", DIR, () >>
}
