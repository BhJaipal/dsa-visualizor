"use client";

import { useTheme } from "next-themes";
import Image from "next/image";

export function GetTheme() {
	const { resolvedTheme, forcedTheme } = useTheme();

	let mode: "light" | "dark" = "dark";
	if (forcedTheme) {
		if (forcedTheme == "dark" || forcedTheme == "light") {
			mode = forcedTheme;
		}
	} else if (resolvedTheme) {
		if (resolvedTheme == "dark" || resolvedTheme == "light") {
			mode = resolvedTheme;
		}
	}

	if (!forcedTheme && !resolvedTheme) {
		if (!window.matchMedia("(prefers-color-scheme: dark)").matches) {
			mode = "light";
		}
	}
	return mode;
}

export function Logo({ title }: { title: string }) {
	return (
		<Image
			alt={title}
			src={
				GetTheme() == "dark"
					? "/applogo-dark.svg"
					: "/applogo-light.svg"
			}
			height={50}
			width={100}
		/>
	);
}

export default function Header({ title }: { title: string }) {
	return (
		<header className="w-full border-b-slate-200 dark:border-b-slate-800 border-b-2 fixed z-10 backdrop-blur-lg px-10 grid grid-cols-5 py-3">
			<div></div>
			<Logo title={title} />
		</header>
	);
}
