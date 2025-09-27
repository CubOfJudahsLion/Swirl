/// <reference path="./base-game.d.ts" />


/** Main interface **/
interface SWIRLGame extends BaseGame {
    /**
     * Starts the loading process. This version of the call joins
     * 'init' and 'load' from the 'Game' interface. This is because
     * 'load' is * expected to load the WebAssembly module, which
     *  returns the actual Rust-side implementation of SWIRLGame.
     * @param eventHandler store this reference you will need it.
     * @param gelGame The external interface to this game, mostly
     *        passed so the SWIRL game can set its swirl_game property.
     * @param language 'en', 'es', etc.
     * @param allsymbols a string with all the symbols 'ABCDEFGXWZ'
     * @param initialReels the reels the machine has to display when
     *        it finishes loading
     * @param fullReels the entire reels for animation
     * @param allcoins for future use - for now it will always be NULL
     * @param linepaths all the line paths ['12321','32123','11111',etc]
     * @param translations an array of tuples where each tuple looks like
     *        [Scode,Value]
     * @returns a promise that resolves when the loading process is over
     * @category for Initialization and loading
     */
    initLoad(
        eventHandler: GameEventHandler,
        language    : string,
        allsymbols  : string,
        initialReels: string[],
        fullReels   : string[],
        allcoins    : string[]|null,
        linepaths   : string[],
        translations: Record<string, string>
    ): Promise<void>;

    /**
     * Notifies the rendering engine to switch to a layout for a
     * different aspect ratio. This is part of the machinery
     * for multiple-layout management and responsiveness in
     * SWIRL canvas-based rendering.
     * Do notice that ratio-switching is a heavy operation that
     * modifies parts of the scene-graph internally. Do not call
     * this function wantonly.
     * @param ratio the new aspect ratio.
     * @category responsive apps
     */
    aspectRatioChanged(ratioIndex: number): void;
}
