import { JSONEditor } from "vanilla-jsoneditor";

import("parser-wasm")
  .then((parser) => {
    const button = document.querySelector("button");
    const editor = new JSONEditor({
      target: document.getElementById("jsoneditor"),
      props: {
        readOnly: true,
        content: {}
      }
    });

    button.addEventListener("click", () => {
      const input = document.querySelector("textarea");
      const result = parser.deserialize_inp(input.value);

      // set json
      editor.set({ json: result });
    });
  })
  .catch(console.error);
