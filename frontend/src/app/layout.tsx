import "./globals.css";
import type { Metadata } from "next";
import Navbar from "@/components/Navbar";

export const metadata: Metadata = {
  title: "Rusty Chat",
  description: "Login and chat with WebSockets",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <Navbar/>
      <body className="bg-gray-100 text-gray-900 p-6">{children}</body>
    </html>
  );
}