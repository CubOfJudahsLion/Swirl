const allSymbols    =   "BFHGIAEDCJXW";
const firstReels    =   "EBW,GEW,EBX,XCD,DEJ".split(',');
const fullReels     =   [   "ACDEIWBGHFACDEIJBGHFACDEWIBGHFAJXCDEIBWAGHFCDEJBIAGHFCDEBWXIAJGHFCDEBAIGHFWCDEJBAIGHFCDEBXAJWGHFIBCDEAGHFIBJCDEWAXGHFBICDEAJGHFB"
                        ,   "AJGEHWBCDFAJGEHIBCDFAJGEWHBCDFAIXJGEHBWACDFJGEIBHACDFJGEBWXHAICDFJGEBAHCDFWJGEIBAHCDFJGEBXAIWCDFHBJGEACDFHBIJGEWAXCDFBHJGEAICDFB"
                        ,   "AJGEIWBCDFAJGEIHBCDFAJGEWIBCDFAHXJGEIBWACDFJGEHBIACDFJGEBWXIAHCDFJGEBAICDFWJGEHBAICDFJGEBXAHWCDFIBJGEACDFIBHJGEWAXCDFBIJGEAHCDFB"
                        ,   "AJGEHWBCDFAJGEHIBCDFAJGEWHBCDFAIXJGEHBWACDFJGEIBHACDFJGEBWXHAICDFJGEBAHCDFWJGEIBAHCDFJGEBXAIWCDFHBJGEACDFHBIJGEWAXCDFBHJGEAICDFB"
                        ,   "ACDEJWBGHFACDEJIBGHFACDEWJBGHFAIXCDEJBWAGHFCDEIBJAGHFCDEBWXJAIGHFCDEBAJGHFWCDEIBAJGHFCDEBXAIWGHFJBCDEAGHFJBICDEWAXGHFBJCDEAIGHFB"
                        ];
const coinValues    =   "$1,$2,$5,$10".split(',');
const linePaths     =   (   "22222,11111,33333,12321,32123,11211,33233,21112,"
                        +   "23332,11233,33211,21232,23212,12121,32323,12221,"
                        +   "32223,21212,23232,22122,22322,13331,31113,13131,31313"
                        ).split(',');
//let autoplayValues  =   "10,20,50,100".split(',');
const balance       =   "$12,312.21";


class GameEventHandler {
    constructor(game, allSymbols) {
        this.game       = game;
        this.allSymbols = allSymbols;
    }

    onSpinClicked() {
        this.game.setSpinEnabled(false);
        for(reel = 1, reel == 5, ++reel)
            setTimeout(
                (thisReel) => {
                    this.game.reelSpin(thisReel);
                    if(thisReel == 5)
                        this.game.setStopEnabled(true);
                },
                (reel - 1) * 75,
                reel
            );
    }

    onStopClicked() {
        if(!this.endReels) {
            console.error("Clicked stop but no reels spinning.");
        }
        else {
            let allL
        }
    }

    updateLoadingProgressBar(position):void {
        console.info("Loaded " + position + "%");
    }

    onBetMaxClicked() {}
    changeCoinValue(_changeAction) {}
    changeLinesBet(_changeAction) {}
    changeLineCoins(_changeAction) {}
    changeTotalBet(_changeAction) {}
}

let game            =   new gameLibrary.GameApp();
let handler         =   new GameEventHandler(game);
game.init(handler);
game.load("en", firstReels, fullReels, coinValues, linePaths, {})
    .then(_ => {
        nad
    });
