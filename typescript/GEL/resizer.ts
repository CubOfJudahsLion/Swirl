/// Module  :   SWIRL/GEL HTML container resizing scripting
/// Purpose :   Scales the page through resizes to keep the canvas
///             visible and focused. Scaling depends on a series
///             of aspect ratios provided by the SWIRL binary.
/// Version :   0.1.0
/// Author  :   Alexander Feterman-Naranjo
/// Created :   Mon 04/18/2022 11:13 AM
/// Modified:   Mon 07/04/2022 05:44 PM


/// <reference path="./swirl-gel.ts"/>

//  Previous ratio. If different from current after sampling,
//  the watchdog function will perform resizing.
let prevHeight      : number            =   NaN;
let prevWidth       : number            =   NaN;

//  The item to be resized. In this case, the DIV containting both
//  background and SWIRL-used canvas. The name is a SWIRL convention.
const holder        : HTMLDivElement    =   document.querySelector("div#backgroundHolder")
                                        ??  (() => {
                                                throw "Container DIV not found";
                                            })();

//  Canvas obtained from the document. The name is a SWIRL convention.
const canvas        : HTMLCanvasElement =   document.querySelector("canvas#renderTarget")
                                        ??  (() => {
                                                throw "Canvas not found";
                                            })();

// //  Aspect ratio of the canvas itself. In case we need it.
// const canvas_ratio  = 1016/796;

//  Sorted list of aspect ratios, provided by the SWIRL WASM. In their absence,
//  the resizing function does not operate.
//  Aspect ratios cover the interval from themselves (inclusive) to the next greater
//  ratio (exclusive.) The exception is, of course, the first ratio, which covers
//  all smaller ratios as well.
let aspectRatios        : [number, number, number][]|null =   null;

//  Keeper of the index of the previous aspect ratio. When it changes,
//  the SWIRL binary must be notified of said change.
let previousRatioIndex  : number            =   NaN;

//  Sets the list of aspect ratios to be contemplated for browser sizes.
//  This function should be called by the SWIRL WASM binary, and just
//  once. In fact, second and further calls will just return false.
function setAspectRatios(ratios: [number, number, number][]): boolean {
    if(aspectRatios != null)
        return false;
    //
    function isSorted(nums: [number, number, number][]): boolean {
        for(let i = 0; i + 1 < nums.length; ++i)
            if(<number>(<number[]>nums[i])[2] > <number>(<number[]>nums[i + 1])[2])
                return false;
        return true;
    }
    //
    if(
            ratios instanceof Array
        &&  ratios.length > 0
        &&  ratios.every(tuple =>
                    tuple instanceof Array
                &&  tuple.every(num =>
                            typeof num === "number"
                        &&  !isNaN(num)
                        &&  isFinite(num)
                    )
            )
        &&  isSorted(ratios)
    )
    {
        aspectRatios = ratios;
        console.log("Setting aspect ratios to: ", aspectRatios);
        resize();
        setInterval(resize, 500);
        console.log("Aspect ratios after resizing and interval-setting: ", aspectRatios);
        return true;
    }
    return false;
}

/// The actual resizing function. It runs periodically, but
/// it won't do anything until the aspect ratios are set.
function resize(): void {
    //  Only act if a list of ratios is given, the wasm_bindgen
    //  component is loaded, and dimensions have changed.
    if(     aspectRatios != null
        &&  wasm_bindgen != null
        &&  (window.innerHeight != prevHeight || window.innerWidth != prevWidth)
    )
    {
        //  Record the ratio.
        const viewportRatio     =   window.innerWidth / window.innerHeight;
        //  Seek the ratio stop that is bigger or equal to the calculated ratio.
        //  (Or stick under the minimum if lower.)
        let ratioIndex: number  =   0;
        while(ratioIndex + 1 < aspectRatios.length && viewportRatio > <number>(<number[]>aspectRatios[ratioIndex])[2])
            ++ratioIndex;
        //  Tell the WASM component to use the new aspect ratio (layout change)
        if(ratioIndex !== previousRatioIndex) {
            previousRatioIndex = ratioIndex;
            wasm_bindgen.aspectRatioChanged(ratioIndex);
        }
        //  Now rescale and center the canvas+background holder.
        //const listRatio         =   <number>(<number[]>aspectRatios[ratioIndex])[2];
        //let scale               =   viewportRatio >= listRatio
        //                                ?   window.innerHeight / <number>(<number[]>aspectRatios[ratioIndex])[1]
        //                                :   window.innerWidth  / <number>(<number[]>aspectRatios[ratioIndex])[0]
        //                                //?   window.innerHeight / canvas.height
        //                                //:   window.innerWidth  / canvas.width
        const xScale            =   window.innerWidth  / <number>(<number[]>aspectRatios[ratioIndex])[0];
        const yScale            =   window.innerHeight / <number>(<number[]>aspectRatios[ratioIndex])[1];
        const scale             =   Math.min(xScale, yScale);
        holder.style.transform  =   `translate(-50%, -50%) scale(${scale})`;
        prevHeight              =   window.innerHeight;
        prevWidth               =   window.innerWidth;
    }
}

// //  The resizing function runs every half second.
// //  Of course, it does nothing until all conditions are met.
//  OVERRIDEN. No longer. Now it's activated by loading the ratios.
// setInterval(resize, 500);
