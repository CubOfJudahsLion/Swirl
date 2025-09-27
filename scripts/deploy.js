"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = require("fs");
const SRC_DIR = "./out/";
const DEST_DIR = "//newgames/HTML5/TestWASM/";
const filesToCopy = [`${SRC_DIR}index.html`,
    `${SRC_DIR}slots_wasm_test_bg.wasm`,
    `${SRC_DIR}slots_wasm_test.js`,
    `${SRC_DIR}css/index.css`,
    `${SRC_DIR}fonts/LHFTonicLIVER.woff`,
    `${SRC_DIR}img/bkgd.png`,
    `${SRC_DIR}img/interface-mockup.png`,
    `${SRC_DIR}img/play-button-click.png`,
    `${SRC_DIR}img/play-button-normal.png`,
    `${SRC_DIR}img/play-button-over.png`
];
filesToCopy.forEach(file => {
    const targetFile = file.replace(new RegExp(`^${SRC_DIR.replace(/\./g, "\.")}`), DEST_DIR);
    const targetDir = targetFile.replace(/\/[^\/]+$/, "");
    try {
        const stat = fs_1.statSync(targetDir);
        if (!stat.isDirectory())
            throw null;
    }
    catch (_a) {
        fs_1.mkdirSync(targetDir);
    }
    fs_1.copyFileSync(file, file.replace(new RegExp(`^${SRC_DIR.replace(/\./g, "\.")}`), DEST_DIR));
});
