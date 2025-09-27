type ChangeAction = "+" | "-" | number;

interface GameEventHandler {
    onAutoPlayClicked(): void
    onAutoPlayConfigSelected(): void;
    onAutoPlayOptionSelected(value: number): void;

    /**
     * Event raised when the player clicks the BETMAX button.
     */
    onBetMaxClicked(): void;

    /**
     * Event raised when the player clicks the coin-value button
     * @param to either '+', '-' or a specific number indicating if it is the #1 coin, the #2 coin, etc. (NOT the coin value)
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    onCoinValueClicked(to: ChangeAction, circular?: false): void;

    /**
     * Event raised when the player clicks the coins-per-line button
     * @param to either '+', '-' or a specific number indicating the number of coins per line
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    onLineCoinsClicked(to: ChangeAction, circular?: false): void;

    /**
     * Event raised when the player clicks the lines-bet button or a selector
     * @param to either '+', '-' or a specific number indicating the number of lines
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    onLinesBetClicked(to: ChangeAction, circular?: false): void;

    /**
      * Event raised when the player clicks the STOP button
      * @returns void
      */
    onPayoutTableClicked(): void

    /**
     * Event raised when the player clicks the SPIN button.
     */
    onSpinClicked(): void;

    /**
     * Event raised when the player clicks the STOP button
     */
    onStopClicked(): void;

    /**
     * Event raised when the player clicks the TotalBet button
     * @param to either '+', '-' or a specific number indicating the number of coins per line
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    onTotalBetClicked(to: ChangeAction, circular?: false): void;

    /**
     * Event raised as the game is loading the assets. 
     * @param position must be between 0 and 100. If you pass -1 the wrapper hides the progress bar (the game must show its own).
     */
    updateLoadingProgressBar(position: number): void;
}
