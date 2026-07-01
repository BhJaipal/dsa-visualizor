"use client";

import { useTheme } from "next-themes";
import Image from "next/image";

export function GetTheme() {
	const { theme } = useTheme();

	let mode: "light" | "dark" = "dark";
	if (theme) {
		if (theme == "dark" || theme == "light") {
			mode = theme;
		}
	}
	console.log(mode);
	return mode;
}

export default function Logo({ title }: { title: string }) {
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
