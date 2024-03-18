import { useMemo, useState } from "react";
import { markdownx } from "markdownx";
import "./App.css";

function App() {
  const [raw, setRaw] = useState("");
  const result = useMemo(() => markdownx(raw), [raw]);
  return (
    <div className="app">
      <textarea
        name="editor"
        cols={30}
        rows={10}
        value={raw}
        onChange={(e) => setRaw(e.target.value)}
      />
      <div dangerouslySetInnerHTML={{ __html: result }} />
    </div>
  );
}

export default App;
