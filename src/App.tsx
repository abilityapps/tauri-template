import "./App.css";

import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";
import { ThemeToggle } from "./components/theme-toggle";
import { invoke } from "@tauri-apps/api/core";
import reactLogo from "./assets/react.svg";
import { useState } from "react";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		setGreetMsg(await invoke("greet", { name }));
	}

	return (
		<div className="flex flex-col items-center justify-center min-h-screen text-center">
			<h1 className="text-3xl font-bold mb-8">Welcome to Tauri!</h1>

			<div className="flex gap-8 justify-center items-center mb-6">
				<a href="https://vitejs.dev" target="_blank" rel="noreferrer">
					<img src="/vite.svg" alt="Vite logo" className="w-24 h-24" />
				</a>
				<a href="https://tauri.app" target="_blank" rel="noreferrer">
					<img src="/tauri.svg" alt="Tauri logo" className="w-24 h-24" />
				</a>
				<a href="https://reactjs.org" target="_blank" rel="noreferrer">
					<img src={reactLogo} alt="React logo" className="w-24 h-24" />
				</a>
			</div>

			<p className="mb-6">
				Click on the Tauri, Vite, and React logos to learn more.
			</p>

			<div className="mb-6">
				<ThemeToggle />
			</div>

			<form
				onSubmit={(e) => {
					e.preventDefault();
					greet();
				}}
				className="flex items-center mb-4"
			>
				<Input
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
					className="mr-2 px-4 py-2 border rounded"
				/>
				<Button
					type="submit"
					className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
				>
					Greet
				</Button>
			</form>

			<p>{greetMsg}</p>
		</div>
	);
}

export default App;
