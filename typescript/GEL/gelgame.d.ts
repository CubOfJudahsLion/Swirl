/// <reference path="./base-game.d.ts" />


/** Main interface **/
interface GELGame extends BaseGame {
    /**
     * Starts the loading process. Most games have the lines hardcoded but this allows for dynamic line loading.
     * @param language 'en', 'es', etc.
     * @param allsymbols a string with all the symbols 'ABCDEFGXWZ'
     * @param initialReels the reels the machine has to display when it finishes loading
     * @param fullReels the entire reels for animation
     * @param allcoins for future use - for now it will always be NULL 
     * @param linepaths all the line paths ['12321','32123','11111',etc]
     * @param translations an array of tuples where each tuple looks like [Scode,Value]
     * @returns a promise that resolves when the loading process is over
     * @category for Initialization and loading
     */
    load(
        eventHandler: GameEventHandler,
        language    : string,
        allsymbols  : string,
        initialReels: string[],
        fullReels   : string[],
        allcoins    : string[],
        linepaths   : string[],
        translations: Record<string, string>
    ): Promise<void>;
}
