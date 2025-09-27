interface GameLoading {
    /**
     * Stops any animation started with animateBalance() and sets the balance to a specific value
     * @param balance the specific balance we want e.g. $123.32
     * @category for Balance and Winnings
     */
    updateBalance(balance: string): void; // this stops any ongoing animation due to animateBalance()
}


interface GameReels {
    /**
     * Starts animating a reel
     * @param reel Reel number e.g. 1,2,3,4 or 5 (or more if the machine has more than 5 reels!)
     * @category for Spinning Animation
     */
    reelSpin(reel: number): void;

    /**
     * Stops animating a reel and shows the final symbols
     * This function will only be called if there reel is spinning
     * @param reel Reel number e.g. 1,2,3,4 or 5 (or more if the machine has more than 5 reels!)
     * @param symbols the final symbols 'ABCWWW'
     * @param forced forced stop
     * @param flags for future use, use null for now
     * @category for Spinning Animation
     */
    reelStop(reel: number, symbols: string, forced: boolean, flags: string[]|null): Promise<void>; // flags can be null, e.g. this.reelStop(1, 'ABCWWW', false, null); 
}


interface GameButtons {
    /**
     * Enables the Spin and the BetMax button
     * @param enabled - true for enabled, false for disabled
     * @category for Buttons
     */
    setSpinEnabled(enabled: boolean): void;

    /**
     * Enables the "Bet Change" buttons
     * - changeLinesBet has to be used for the selector buttons too
     * - The TotalBet button needs to be enabled like this: 
     * - TotalBet.enabled = changeCoinValue || changeLineCoins || changeLinesBet
     * @param enabled true for enabled, false for disabled
     * @category for Buttons
     */
    setChangeEnabled(changeCoinValue: boolean, changeLineCoins: boolean, changeLinesBet: boolean, changeTotalBet: boolean): void;

    /**
     * Enables or disables the Stop button (which moves independently from all the other buttons)
     * @param enabled true for enabled, false for disabled
     * @category for Buttons
     */
    setStopEnabled(enabled: boolean): void;       // applies to the STOP button only
}


interface GameBetting {
    /**
     * Only relevant to games that show TotalBet. Ignore otherwise
     * @param totalbet The total bet amount e.g. $132.00
     * @category for Bet Configuration
     */
    updateTotalBet(totalbet: string): void;

    /**
     * Updates the coin value displayed to the user
     * @param linecoins 
     * @category for Bet Configuration
     */
    updateCoinValue(coinvalue: string): void;

    /**
     * Updates the number of coins per line displayed to the user
     * @param linecoins 
     * @category for Bet Configuration
     */
    updateLineCoins(linecoins: number): void;

    /**
     * Updates the number of lines bet displayed to the user
     * - If the game doesn't have a label displaying the number of lines, then ignore it
     * - ???? If the game has selectors, this is the time to update them too. ???
     * @param linesbet 
     * @category for Bet Configuration
     */
    updateLinesBet(linesbet: number): void; 
}


interface GamePayouts {
    // /**
    //  * Animates a Winning Symbol. 
    //  * - If the symbol was playing a teaser animation, the teaser animation has to go away so we can play this other animation.
    //  * - If the symbol was already playing a winning animation, it has to restart it.
    //  * @param reel reel number (e.g. 1,2,3,4,5)
    //  * @param row row number (e.g. 1,2,3)
    //  * @returns a promise that resolves when the animation is over
    //  * @category for Symbol Animations
    //  */
    // animateSymbolWinner(reel: number, row: number): Promise<void>;

    /**
     * Stops any animation started with animateWinnings() and clears the winnings. 
     * - If any visual elements are related to the winnings, they need to hide too.
     * - For most games this function could be implemented as updateWinnings("")
     * @category for Balance and Winnings
     */
    clearWinnings(): void;

    // /**
    //  * Hides (and possibly iterrupts!) the animation for the specified line
    //  * @param linepath the line to hide e.g. '12321'
    //  * @category for Payout Animations
    //  */
    // hideLinePayout(linepath: string): void;

    // /**
    //  * Hides (and possibly iterrupts!) the animation for the scatter payout
    //  * @category for Payout Animations
    //  */
    // hideScatterPayout(): void;

    // /**
    //  * Enable or disable symbol dimming
    //  * - This method will NEVER get called for a symbol that's animating. If a symbol is animating a call to stopSymbolAnimation() will be made first
    //  * @param reel reel number (e.g. 1,2,3,4,5)
    //  * @param row row number (e.g. 1,2,3)
    //  * @param dimming whether to dim this symbol. 
    //  * @category for Symbol Animations
    //  */
    // setSymbolDimming(reel: number, row: number, dimming: boolean): void

    // /**
    //  * Shows a line payout and if there's an animation, it plays it
    //  * @param linepath the line to show e.g. '12321'
    //  * @param label the label to show as the payout e.g. $40. It can be null (if the payout was in freespins or bonus)
    //  * @param direction for future use
    //  * @returns null if there's no animation, or a promise if there is
    //  * @category for Payout Animations
    //  */
    // showLinePayout(linepath:string, label:string, direction:number): null|Promise<void>

    // /**
    //  * Shows a scatter payout
    //  * @param winamts a formatted array of winnings for the animation ['$2.00','$3.00','$4.00','$5.00',etc]
    //  * @returns a promise that resolves when the animation is over
    //  * @category for Payout Animations
    //  */
    // showScatterPayout(winamts:string[]):Promise<void>

    // /**
    //  * Stops any ongoing animation with this symbol
    //  * @param reel reel number (e.g. 1,2,3,4,5)
    //  * @param row row number (e.g. 1,2,3)
    //  * @param dimming whether to dim this symbol. 
    //  * @category for Symbol Animations
    //  */
    // stopSymbolAnimation(reel:number, row:number):void

    // /**
    //  * Stops any animation started with animateWinnings() and sets the winnings to a specific value
    //  * @param winamt the specific winnings we want e.g. $50.00
    //  * @category for Balance and Winnings
    //  */
    // updateWinnings(winamt:string):void; 
}


interface GameFreeSpins {
    // /** Puts the machine in a special mode (if the machine has this feature)
    //  * @param name special mode
    //  * @param flags optional flags (specific to this mode) or null
    //  * @category for FreeSpins and Machine Modes
    //  */
    // enterCustomMode(name:string, flags:string[]):Promise<void>;

    // /** Puts the machine in freespin mode
    //  * @param freespins the initial value for the freespins counter. Then it gets updated by updateFreeSpins()
    //  * @category for FreeSpins and Machine Modes
    //  */
    // enterFreeSpinMode(freespins:number):Promise<void>;

    // /** Returns the machine to "Normal" mode (not freespins)
    //  * @category for FreeSpins and Machine Modes
    //  */
    // exitMode():Promise<void>;

    // /**
    //  * Updates the freespins counter
    //  * @param freespins the number of remaining freespins.
    //  * @category for FreeSpins and Machine Modes
    //  */
    // updateFreeSpins(freespins:number):void;
}

interface GameSounds {
    // /**
    //  * Changes the volume (no digas!)
    //  * @param volume number between 0.0 and 1.0
    // */
    // changeVolume(volume:number):void;

    // //  Plays a sound.
    // //  @param name the name of the sound (game-specific)
    // //  @param loop whether it should loop or not
    // //  @returns void    
    // playSound(name: string, loop: boolean): void

    // stopSound(name: string): void
    // //  Stops a sound that was previously played by playSound()
    // //  @param name the name of the sound (game-specific)
    // //  @returns void
}


interface GameAutoPlay {
    // //  Resets the counter in the AutoPlay button back to the autoplay icon
    // //  @returns void
    // clearAutoPlayCounter(): void;

    // //  Hides the autoPlay menu
    // //  @returns void
    // hideAutoPlayMenu(): void;

    // //  Shows the autoPlay menu
    // //  @returns void
    // showAutoPlayMenu(): void

    // //  Updates the counter in the AutoPlay button
    // //  @param value the counter
    // //  @returns void
    // updateAutoPlayCounter(value: number): void
}


interface GameFancy {
    // /**
    //  * Uber, duper, super, mega win!!!
    //  * - The game will always wait for this animation to finish.
    //  * @param level 1 for super, 2 for mega, 3 for super-mega, etc. Depends on the game.
    //  * @param winamts a formatted array of winnings ['$2.00','$3.00','$4.00','$5.00',etc]
    //  * @returns a promise that resolves when the animation is over
    //  * @category for Other Animations
    //  */
    // animateBigWin(level:number, winamts:string[]):Promise<void>;

    // /**
    //  * Plays a custom animation for this particular game
    //  * - The game will always wait for this animation to finish.
    //  * @param animation the name of the animation
    //  * @param flags optional flags (specific to this animation) or null
    //  * @returns a promise that resolves when the animation is over
    //  * @category for Other Animations
    //  */
    // animateCustom(animation:string, flags:string[]):Promise<void>;

    // /**
    //  * Morphs this symbol into a different one
    //  * @param reel reel number (e.g. 1,2,3,4,5)
    //  * @param row row number (e.g. 1,2,3)
    //  * @param newsymbol the new symbol
    //  * @category for Symbol Animations
    //  */
    // animateSymbolChange(reel:number, row:number, newsymbol:string):Promise<void>;

    // /**
    //  * Animates a teaser Symbol.
    //  * @param reel - reel number (e.g. 1,2,3,4,5)
    //  * @param row - row number (e.g. 1,2,3)
    //  * @returns a promise that resolves when the animation is over
    //  * @category for Symbol Animations
    //  */
    // animateSymbolTeaser(reel:number, row:number):Promise<void>;

    // /**
    //  * Updates the internal symbols used to animate each reel
    //  * @param symbols array with symbols for each reel (e.g. ['ABCDEFGDCFAXWWWXABCDEDCBA', ..., 'ABCDEFGDCFAXWWWXABCDEDCBA'])
    //  * @category for Spinning Animation
    //  */
    // updateReelSymbols(symbols:string[]):void;
}


/** Main interface **/
interface BaseGame extends
    GameLoading, GameReels, GameButtons, GameBetting, GamePayouts,
    GameFreeSpins, GameSounds, GameAutoPlay, GameFancy
{ }
