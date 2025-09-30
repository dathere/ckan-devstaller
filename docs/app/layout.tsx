import "@/app/global.css";
import { RootProvider } from "fumadocs-ui/provider";
import localFont from "next/font/local";

const inter = localFont({ src: "../lib/inter.ttf" });

export default function Layout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className={inter.className} suppressHydrationWarning>
      <body className="flex flex-col min-h-screen">
        <RootProvider>{children}</RootProvider>
      </body>
    </html>
  );
}
