import("parser-wasm")
  .then((parser) => {
    const button = document.querySelector("button");
    addEventListener("click", () => {
      const input = document.querySelector("textarea");
      const result = parser.deserialize_inp(input.value);
      console.log(result);
    });
  })
  .catch(console.error);
