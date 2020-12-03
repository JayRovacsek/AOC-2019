dataSubmit.addEventListener("click", () => {
  const wasm = require("aoc_2020");
  const vals = [...document.getElementById("formInput").elements]
    .filter((x) => x.value !== "")
    .map((x) => x.value);
  const result = wasm.solve(vals[0], vals[1]);
  document.getElementById("formOutput").innerHTML = result;
});
