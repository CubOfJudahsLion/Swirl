/// Module  :   SWIRL/GEL JavaScript bridge.
/// Purpose :   Direct interface serving GEL calls.
/// Version :   0.1.0
/// Author  :   Alexander Feterman-Naranjo
/// Created :   Tue 02/22/2022 01:08 PM
/// Modified:   Mon 04/18/2022 11:12 AM
/// Notes   :   This implementation assumes wasm-pack
///             was called with `-t no-modules`.


/// <reference path="./game-event-handler.d.ts" />
/// <reference path="./gel-game.d.ts" />
/// <reference path="./swirl-game.d.ts" />


interface WasmBindgen extends SWIRLGame {
    (wasm_loc?: string): Promise<unknown>;
    init(): void;
}

declare let wasm_bindgen: WasmBindgen;


class GameBridge implements GELGame {
    private evh         : GameEventHandler|null = null;
    private instanced   : boolean               = false;

    constructor() {}

    init(evh: GameEventHandler): void {
        this.evh = evh;
    }

    private static genPromiseErr(errMsg: string): Promise<void> {
        console.error(errMsg);
        return Promise.reject(errMsg);
    }

    load(
        language    : string,
        allsymbols  : string,
        initialReels: string[],
        fullReels   : string[],
        allcoins    : string[]|null,
        linepaths   : string[],
        translations: Record<string, string>
    ): Promise<void>
    {
        if(window.WebAssembly == undefined)
            return GameBridge.genPromiseErr("GameBridge.load(): Error: this browser does not support WebAssembly.");
        else if(this.evh == null)
            return GameBridge.genPromiseErr("GameBridge.load(): Error: init() has not been called.");
        else {
            let evh: GameEventHandler = this.evh;
            return wasm_bindgen("game.wasm")
                .then(_ => {
                    wasm_bindgen.init();
                    return wasm_bindgen.initLoad(
                        evh,
                        language,
                        allsymbols,
                        initialReels,
                        fullReels,
                        allcoins,
                        linepaths,
                        translations
                    )
                })
                .then(_ => {
                    this.instanced = true;
                    console.groupCollapsed("Translations");
                    for(let tr in translations) {
                        console.log(`[${tr}] = "${translations[tr]}"`);
                    }
                    console.groupEnd();
                    return Promise.resolve();
                });
        }
    }

    updateBalance(balance: string): void {
        if(!this.instanced)
            console.error("GameBridge.updateBalance(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.updateBalance(balance);
    }

    reelSpin(reel: number): void {
        if(!this.instanced)
            console.error("GameBridge.reelSpin(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.reelSpin(reel);
    }

    reelStop(reel: number, symbols: string, forced: boolean, flags: string[]|null): Promise<void> {
        if(!this.instanced)
            return GameBridge.genPromiseErr("GameBridge.reelStop(): must .init() and .load() before any other operations.");
        return wasm_bindgen.reelStop(reel, symbols, forced, flags);
    }

    setStopEnabled(enabled: boolean): void {
        if(!this.instanced)
            console.error("GameBridge.setStopEnabled(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.setStopEnabled(enabled);
    }

    setSpinEnabled(enabled: boolean): void {
        if(!this.instanced)
            console.error("GameBridge.setSpinEnabled(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.setSpinEnabled(enabled);
    }

    setChangeEnabled(changeCoinValue: boolean, changeLineCoins: boolean, changeLinesBet: boolean, changeTotalBet: boolean): void {
        if(!this.instanced)
            console.error("GameBridge.setChangeEnabled(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.setChangeEnabled(changeCoinValue, changeLineCoins, changeLinesBet, changeTotalBet);
    }

    updateCoinValue(coinvalue: string): void {
        if(!this.instanced)
            console.error("GameBridge.updateCoinValue(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.updateCoinValue(coinvalue);
    }

    updateLineCoins(linecoins: number): void {
        if(!this.instanced)
            console.error("GameBridge.updateLineCoins(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.updateLineCoins(linecoins);
    }

    updateLinesBet(linesbet: number): void {
        if(!this.instanced)
            console.error("GameBridge.updateLinesBet(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.updateLinesBet(linesbet);
    }

    updateTotalBet(totalBet: string): void {
        if(!this.instanced)
            console.error("GameBridge.updateTotalBet(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.updateTotalBet(totalBet);
    }

    clearWinnings(): void {
        if(!this.instanced)
            console.error("GameBridge.clearWinnings(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.clearWinnings();
    }

    aspectRatioChanged(ratio_index: number): void {
        if(!this.instanced)
            console.error("GameBridge.aspectRatioChanged(): must .init() and .load() before any other operations.");
        else
            wasm_bindgen.aspectRatioChanged(ratio_index);
    }
}


namespace gameLibrary {
    export let GameApp: { new (): GELGame; } = GameBridge;
}
