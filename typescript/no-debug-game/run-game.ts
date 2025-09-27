/// <reference path="../GEL/game-event-handler.d.ts" />
/// <reference path="../GEL/gel-game.d.ts" />
/// <reference path="../GEL/swirl-game.d.ts" />
/// <reference path="../GEL/swirl-gel.ts" />


const allSymbols    : string        =   "BFHGIAEDCJXW";

const firstReels    : string[]      =   ["EBW", "GEW", "EBX", "XCD", "DEJ"];

const fullReels     : string[]      =   [   "ACDEIWBGHFACDEIJBGHFACDEWIBGHFAJXCDEIBWAGHFCDEJBIAGHFCDEBWXIAJGHFCDEBAIGHFWCDEJBAIGHFCDEBXAJWGHFIBCDEAGHFIBJCDEWAXGHFBICDEAJGHFB"
                                        ,   "AJGEHWBCDFAJGEHIBCDFAJGEWHBCDFAIXJGEHBWACDFJGEIBHACDFJGEBWXHAICDFJGEBAHCDFWJGEIBAHCDFJGEBXAIWCDFHBJGEACDFHBIJGEWAXCDFBHJGEAICDFB"
                                        ,   "AJGEIWBCDFAJGEIHBCDFAJGEWIBCDFAHXJGEIBWACDFJGEHBIACDFJGEBWXIAHCDFJGEBAICDFWJGEHBAICDFJGEBXAHWCDFIBJGEACDFIBHJGEWAXCDFBIJGEAHCDFB"
                                        ,   "AJGEHWBCDFAJGEHIBCDFAJGEWHBCDFAIXJGEHBWACDFJGEIBHACDFJGEBWXHAICDFJGEBAHCDFWJGEIBAHCDFJGEBXAIWCDFHBJGEACDFHBIJGEWAXCDFBHJGEAICDFB"
                                        ,   "ACDEJWBGHFACDEJIBGHFACDEWJBGHFAIXCDEJBWAGHFCDEIBJAGHFCDEBWXJAIGHFCDEBAJGHFWCDEIBAJGHFCDEBXAIWGHFJBCDEAGHFJBICDEWAXGHFBJCDEAIGHFB"
                                        ];

type Currency                       =   "$";
type CurrencyUnits                  =   1 | 2 | 5 | 10;
type CoinValue                      =   `${Currency}${CurrencyUnits}`;
const coinValues    : CoinValue[]   =   ["$1", "$2", "$5", "$10"];

type LineNumber                     =   1 | 2 | 3;
type LinePath                       =   `${LineNumber}${LineNumber}${LineNumber}${LineNumber}${LineNumber}`;
const linePaths     : LinePath[]    =   [   "22222", "11111", "33333", "12321", "32123"
                                        ,   "11211", "33233", "21112", "23332", "11233"
                                        ,   "33211", "21232", "23212", "12121", "32323"
                                        ,   "12221", "32223", "21212", "23232", "22122"
                                        ,   "22322", "13331", "31113", "13131", "31313"
                                        ];

type Digit                          =   keyof [0,1,2,3,4,5,6,7,8,9] & number;
type NonZeroDigit                   =   Exclude<Digit, 0>;
type OptDigit                       =   `` | `${Digit}`;
type FirstThree                     =   `${Digit}` | `${NonZeroDigit}${Digit}${OptDigit}`;
type OptThree                       =   `` | `,${Digit}${Digit}${Digit}`;
type OptDecs                        =   `` | `.${Digit}${Digit}`;
type Money                          =   `${Currency}${FirstThree}${OptThree}${OptDecs}`;
const balance       : Money         =   "$12,312.21";


class MinimalEventHandler implements GameEventHandler {
    constructor(private game: GELGame) { }

    onSpinClicked() {
        this.game.setSpinEnabled(false);
        for(let reel = 1; reel <= 5; ++reel)
            setTimeout(
                (thisReel) => {
                    this.game.reelSpin(thisReel);
                    if(thisReel == 5) {
                        this.game.setStopEnabled(true);
                        setTimeout(() => this.stopReels(), 2500);
                    }
                },
                (reel - 1) * 100,
                reel
            );
    }

    private stopReels(instant: boolean = false): Promise<void> {
        const endReels  = fullReels.map((reel) => {
            const rLen  = reel.length;
            const start = Math.floor(Math.random() * rLen);
            return [0, 1, 2]
                .map(j => reel.charAt((j + start) % rLen))
                .join();
        });
        if(instant) {
            endReels.forEach((endR, i) => game.reelStop(i, endR, true, null));
            return Promise.resolve();
        }
        else {
            const recurse = (endReels: string[]): Promise<void> => {
                return endReels.length === 0
                    ?   Promise.reject()
                    :   game.reelStop(5 - endReels.length + 1, <string>endReels[0], false, null)
                            .then(() => {
                                const rest = endReels.slice(1);
                                return rest.length == 0
                                    ?   Promise.resolve()
                                    :   recurse(rest);
                            });
            };
            return recurse(endReels);
        }
    }

    onStopClicked() {
        this.stopReels(true);
    }

    updateLoadingProgressBar(position: number): void {
        console.info("Loaded " + position + "%");
    }

    onBetMaxClicked() {}
    changeCoinValue(_changeAction: ChangeAction) {}
    changeLinesBet(_changeAction: ChangeAction) {}
    changeLineCoins(_changeAction: ChangeAction) {}
    changeTotalBet(_changeAction: ChangeAction) {}
}

let game            =   new gameLibrary.GameApp();
let handler         =   new MinimalEventHandler(game);
game.init(handler);
game.load("en", allSymbols, firstReels, fullReels, coinValues, linePaths, {})
    .then(_ => {
    });
