import init, { wasm_main } from "./pkg/open_source_games_catalogue.js";

async function run() {
  await init();
  wasm_main();
}

run();