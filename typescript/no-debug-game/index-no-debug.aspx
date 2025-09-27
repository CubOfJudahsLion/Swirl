<html>
    <head>
        <title>Game Tester</title>
        <style>
            body, div {
                border  : 0;
                padding : 0;
                margin  : 0;
                overflow: hidden;
                position: absolute;
                top     : 0;
                left    : 0;
            }
        </style>
    </head>
    <body>
        <div id="backgroundHolder">
          	<canvas id="renderTarget" width="1611" height="796">
								"Canvas" HTML element not supported.<br/>
								Please update your browser to render this page correctly.
          	</canvas>
        </div>
        <label id="runInfo"></label>
        <script src="wasm-game-loader.js"></script>
        <script src="swirl-gel.js"></script>
        <script src="run-game.js"></script>
    </body>
</html>
