import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "MemoryLineage Inspector",
  description: "Inspect and replay committed private-agent-memory history.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
