import * as wasm from "aoc_2020";

wasm.solve("a", "b");

// import * as wasm from "aoc_2020";
// Everything but IE
window.addEventListener(
  "load",
  function () {
    dataSubmit.addEventListener("click", () => {
      const wasm = require("aoc_2020");
      const vals = [...document.getElementById("formInput").elements]
        .filter((x) => x.value !== "")
        .map((x) => x.value);
      const result = wasm.solve(vals[0], vals[1]);
      console.log(result);
      document.getElementById("output").innerHTML = result;
    });
  },
  false
);

// // IE
// window.attachEvent("onload", function() {
//   dataSubmit.addEventListener("click", () => {
//     const wasm = require("aoc-2020");
//     const vals = [...document.getElementById("formInput").elements]
//       .filter((x) => x.value !== "")
//       .map((x) => x.value);
//     const result = wasm.solve(vals[0], vals[1]);
//     console.log(result);
//     document.getElementById("output").innerHTML = result
//   });
// });
