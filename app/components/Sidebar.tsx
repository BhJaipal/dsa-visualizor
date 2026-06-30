"use client";
import { usePathname } from "next/navigation";

export default function Sidebar() {
	if (usePathname() == "/") {
		return <p></p>;
	}
	return <p>Sidebar</p>;
}
