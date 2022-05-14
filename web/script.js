import init, { gday } from "./pkg/rpl.js";

init().then(() => {
    console.log(gday("world"));
});
