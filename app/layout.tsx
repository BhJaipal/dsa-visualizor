import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { Icon } from "@iconify/react";
import Sidebar from "./components/Sidebar";
import Header from "./components/Header";

const geistSans = Geist({
	variable: "--font-geist-sans",
	subsets: ["latin"],
});

const geistMono = Geist_Mono({
	variable: "--font-geist-mono",
	subsets: ["latin"],
});

const title = "Data Structures and Algorithms Visualizer Platform";
const desc =
	"The DSA Visualizer Platform aims to bridge this gap by providing an interactive environment where users can perform operations, observe execution in real time, analyze complexity, and connect theoretical concepts with practical implementations.";

export const metadata: Metadata = {
	title,
	description: desc,
};
export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html
			lang="en"
			className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}
		>
			<body className="min-h-full flex flex-col">
				<Header title={title} />
				<main className="flex flex-row float-left h-[85vh] justify-center mt-20">
					<Sidebar></Sidebar>
					{children}
				</main>
				<footer className="grid grid-cols-5 py-4 bg-background border-t-slate-200 dark:border-t-slate-800 border-t-2 text-slate-400">
					<div></div>
					<p className="text-muted flex flex-row items-center gap-2">
						<span>
							Built by BhJaipal • © {new Date().getFullYear()}
						</span>
						<a
							href="https://codeberg.org/BhJaipal"
							target="_blank"
							aria-label="GitHub"
						>
							<Icon icon="simple-icons:codeberg" />
						</a>
						<a
							href="https://github.com/BhJaipal"
							target="_blank"
							aria-label="GitHub"
						>
							<Icon icon="simple-icons:github" />
						</a>
					</p>
					<div></div>
					<div></div>
					<p className="text-muted flex flex-row items-center gap-2">
						Source:
						<a
							href="https://github.com/BhJaipal/dsa-visualizor"
							target="_blank"
							aria-label="GitHub"
						>
							<Icon icon="simple-icons:github" />
						</a>
					</p>
				</footer>
			</body>
		</html>
	);
}
