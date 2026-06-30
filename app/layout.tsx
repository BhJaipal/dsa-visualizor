import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import Image from "next/image";

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
				<header className="w-full h-10 bg-[#0e162a] px-10">
					<Image
						alt={title}
						src="/applogo.svg"
						height={40}
						width={100}
					/>
				</header>
				<br />
				<hr className="bg-slate-800" />
				{children}
			</body>
		</html>
	);
}
