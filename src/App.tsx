import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import { Button } from "./components/ui/button";

function App() {
  const [result, setResult] = useState("");
  const [enabled, setEnabled] = useState(false);

  async function net_stat() {
    const response: string = await invoke("net_stat");

    try {
      setResult(JSON.stringify(JSON.parse(response), null, 2));
    } catch {
      setResult(response);
    }
  }

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;
    if (enabled) {
      interval = setInterval(() => {
        net_stat();
      }, 250);
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [enabled]);

  return (
    <div className="container">
      <Button type="button" onClick={() => setEnabled((prev) => !prev)}>
        run net stat ({enabled ? "on" : "off"})
      </Button>

      <pre>{result}</pre>
    </div>
  );
}

export default App;
