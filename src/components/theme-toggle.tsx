import { Moon, Sun } from "lucide-react";

import { Button } from "@/components/ui/button";
import { useTheme } from "@/components/theme-provider";

export function ThemeToggle() {
	const { theme, setTheme } = useTheme();

	return (
		<Button
			variant="outline"
			size="icon"
			onClick={() => setTheme(theme === "light" ? "dark" : "light")}
			aria-label="Toggle theme"
		>
			<Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all duration-150 dark:-rotate-90 dark:scale-0" />
			<Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all duration-150 dark:rotate-0 dark:scale-100" />
		</Button>
	);
}
