var spawn = require("child_process").spawnSync;

var res = spawn("cargo", ["rustc", "--lib", "--features", "check", "--", "-Z", "no-trans"], { stdio: "inherit" }).status;
if (res == 0) spawn("cargo rustc --bin --features check --Z no-trans");

console.log("----------");
