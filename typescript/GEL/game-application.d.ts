/// <reference path="gel-game.d.ts" />

/**
  * GEL instances the game using this call:
  * `let gameApplication = new gameLibrary.GameApp();`
  * where `gameLibrary` is a global.
  */
interface GameLibrary {
    GameApp(): GELGame;
}
