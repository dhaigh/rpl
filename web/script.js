import init, { tokenize } from "./pkg/rpl.js";

init().then(() => {
    console.log(tokenize("world"));
});
