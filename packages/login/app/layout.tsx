import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Login | FROGZONE",
  description: "Login to FROGZONE",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
