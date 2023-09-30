import { JSONEditor } from "vanilla-jsoneditor";

import("parser-wasm")
  .then((parser) => {
    const editor = new JSONEditor({
      target: document.getElementById("jsoneditor"),
      props: {
        readOnly: true,
        content: {}
      }
    });

    const inputElement = document.getElementById("inp");
    const handleFiles = () => {
      inputElement.files[0].text().then((text) => {
        const result = parser.deserialize_inp(text);

        editor.set({ json: result });
      });
    };

    inputElement.addEventListener("change", handleFiles, false);
  })
  .catch(console.error);
