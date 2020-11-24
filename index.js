dataSubmit.addEventListener("click", () => {
  const wasm = require("aoc-2020");
  const vals = [...document.getElementById("formInput").elements]
    .filter((x) => x.value !== "")
    .map((x) => x.value);
  const result = wasm.solve(vals[0], vals[1]);
  console.log(result);
});
