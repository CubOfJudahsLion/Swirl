//! An direct output dump for the development of asset_builder.
use swirl_asset_builder::*;


#[test]
fn early() {
    build_assets!{
        Outermost {
            Inner1 = "inner1.woff",
            Inner2 : InnerEnum2 = "img/lightning-anim/lightning-all-around-" [ .. 35 : 4] ".png",
            Inner3 : InnerEnum3 = "img/lightning-anim/lightning-left-cap-" [0 ..= 21] ".png",
            Inner4 : Inner4Enum {
                FurtherInside = "inner3/further_inside.jpg",
                EvenMoreNesting : EvenMoreNestingEnum {
                    Inner4 = "inner4.again.jpg",
                    Innermost = "inner4/even_more_nesting/innermost.svg",
                },
            },
            Inner5 = "inner4.Mp3",
        }
    }
    println!("{}", temp_output);
}
