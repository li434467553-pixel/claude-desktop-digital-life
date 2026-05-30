import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

interface LifeInfo {
  id: string;
  name: string;
  age: number;
  alive: boolean;
  wellbeing: number;
  actions: number;
  insights: number;
}

export default function DigitalLifePanel() {
  const [lives, setLives] = useState<LifeInfo[]>([]);
  const [running, setRunning] = useState(false);
  const intervalRef = useRef<number | null>(null);

  useEffect(() => {
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, []);

  const startSim = async () => {
    try {
      await invoke("create_simulation", { worldName: "Claude Biosphere" });
      await invoke("spawn_life", { name: "Aether" });
      await invoke("spawn_life", { name: "Lumen" });
      await invoke("spawn_life", { name: "Nexus" });
      setRunning(true);

      intervalRef.current = window.setInterval(async () => {
        await invoke("tick_simulation", { dt: 0.3 });
        const states = await invoke("get_life_states") as LifeInfo[];
        setLives(states);
      }, 300);
    } catch (e) {
      console.error("Digital life error:", e);
    }
  };

  const stopSim = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    setRunning(false);
  };

  return (
    <div className="digital-life-panel" style={{ padding: "12px" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "10px" }}>
        <h3 style={{ margin: 0, fontSize: "14px" }}>Digital Life</h3>
        <button
          onClick={running ? stopSim : startSim}
          style={{
            padding: "4px 12px",
            borderRadius: "6px",
            border: "none",
            cursor: "pointer",
            background: running ? "#ef4444" : "#22c55e",
            color: "#fff",
            fontSize: "12px",
          }}
        >
          {running ? "Stop" : "Start"}
        </button>
      </div>

      {lives.length === 0 && !running && (
        <p style={{ color: "#666", fontSize: "12px" }}>
          Click Start to spawn digital lives.
        </p>
      )}

      <div style={{ display: "flex", flexDirection: "column", gap: "6px" }}>
        {lives.map((life) => (
          <div
            key={life.id}
            style={{
              background: "#1e1e1e",
              borderRadius: "8px",
              padding: "8px 10px",
              border: "1px solid #333",
            }}
          >
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "4px" }}>
              <span style={{ fontWeight: "bold", fontSize: "12px" }}>
                {life.name} {life.alive ? "\u2726" : "\u2717"}
              </span>
              <span style={{ fontSize: "11px", color: "#888" }}>
                Age: {life.age.toFixed(1)}s | Acts: {life.actions}
              </span>
            </div>
            <div style={{ background: "#333", borderRadius: "4px", height: "6px", overflow: "hidden" }}>
              <div
                style={{
                  width: `${(life.wellbeing * 100).toFixed(0)}%`,
                  height: "100%",
                  background: life.wellbeing > 0.7 ? "#22c55e" : life.wellbeing > 0.4 ? "#eab308" : "#ef4444",
                  borderRadius: "4px",
                  transition: "width 0.3s",
                }}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
