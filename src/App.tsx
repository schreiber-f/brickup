import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";

// Hier importieren wir die neu installierten shadcn-Komponenten
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Ruft die Rust-Funktion in Tauri auf
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex min-h-screen w-screen items-center justify-center bg-zinc-950 p-4 text-zinc-50 antialiased selection:bg-zinc-500/30">
      
      {/* Eine moderne Card als Container für deine App */}
      <Card className="w-full max-w-md border-zinc-800 bg-zinc-900/50 backdrop-blur-xl text-zinc-100">
        <CardHeader className="text-center space-y-2">
          <CardTitle className="text-2xl font-bold tracking-tight bg-gradient-to-r from-indigo-400 via-sky-400 to-emerald-400 bg-clip-text text-transparent">
            Tauri + React + shadcn/ui
          </CardTitle>
          <CardDescription className="text-zinc-400 text-sm">
            Klicke auf die Logos, um die Dokumentation zu öffnen.
          </CardDescription>
        </CardHeader>
        
        <CardContent className="space-y-6">
          {/* Die Logo-Sektion mit Tailwind Flexbox & Hover-Effekten */}
          <div className="flex items-center justify-center gap-6 py-2">
            <a href="https://vite.dev" target="_blank" rel="noreferrer" className="transition-transform hover:scale-110">
              <img src="/vite.svg" className="h-14 w-14 drop-shadow-[0_0_15px_rgba(100,108,255,0.3)]" alt="Vite logo" />
            </a>
            <a href="https://tauri.app" target="_blank" rel="noreferrer" className="transition-transform hover:scale-110">
              <img src="/tauri.svg" className="h-14 w-14 drop-shadow-[0_0_15px_rgba(36,199,233,0.3)]" alt="Tauri logo" />
            </a>
            <a href="https://react.dev" target="_blank" rel="noreferrer" className="transition-transform hover:scale-110">
              <img src={reactLogo} className="h-14 w-14 drop-shadow-[0_0_15px_rgba(97,218,251,0.3)]" alt="React logo" />
            </a>
          </div>

          {/* Das umgebaute Formular mit shadcn-Input und shadcn-Button */}
          <form
            className="flex flex-col gap-3"
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <div className="flex gap-2">
              <Input
                id="greet-input"
                type="text"
                value={name}
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Namen eingeben..."
                className="border-zinc-800 bg-zinc-950 text-zinc-100 placeholder:text-zinc-500 focus-visible:ring-sky-500"
              />
              <Button type="submit" className="bg-sky-500 font-medium text-zinc-950 hover:bg-sky-400 cursor-pointer">
                Begrüßen
              </Button>
            </div>
          </form>

          {/* Animierte Ausgabe der Antwort aus Rust */}
          {greetMsg && (
            <div className="rounded-lg border border-emerald-500/20 bg-emerald-500/10 p-3 text-center text-sm font-medium text-emerald-400 transition-all">
              {greetMsg}
            </div>
          )}
        </CardContent>
      </Card>

    </div>
  );
}

export default App;
