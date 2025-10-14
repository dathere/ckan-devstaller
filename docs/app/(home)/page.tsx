/** biome-ignore-all lint/suspicious/noArrayIndexKey: Would need to look into this trivial issue */
"use client";

import defaultMdxComponents from "fumadocs-ui/mdx";
import { cn } from "fumadocs-ui/utils/cn";
import {
  BlocksIcon,
  GitMergeIcon,
  HomeIcon,
  SailboatIcon,
  ZapIcon,
} from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { useState } from "react";
import { buttonVariants } from "@/components/ui/button";
import CkanDevstallerDemo from "./ckan-devstaller-demo.gif";

export default function HomePage() {
  const gridColor =
    "color-mix(in oklab, var(--color-fd-primary) 10%, transparent)";
  const { Card, Cards } = defaultMdxComponents;
  return (
    <>
      <div
        className="absolute inset-x-0 top-[360px] h-[250px] max-md:hidden"
        style={{
          background: `repeating-linear-gradient(to right, ${gridColor}, ${gridColor} 1px,transparent 1px,transparent 50px), repeating-linear-gradient(to bottom, ${gridColor}, ${gridColor} 1px,transparent 1px,transparent 50px)`,
        }}
      />
      <main className="container relative max-w-[1100px] px-2 py-4 z-2 lg:py-8">
        <div
          style={{
            background:
              "repeating-linear-gradient(to bottom, transparent, color-mix(in oklab, var(--color-fd-primary) 1%, transparent) 500px, transparent 1000px)",
          }}
        >
          <div className="relative mb-4">
            <Hero />
          </div>
        </div>
      </main>
    </>
  );
}

function Hero() {
  const { Card, Cards } = defaultMdxComponents;
  return (
    <div className="relative z-2 flex flex-col border-x border-t bg-fd-background/80 px-4 pt-12 max-md:text-center md:px-12 md:pt-16 [.uwu_&]:hidden overflow-hidden">
      <div
        className="absolute inset-0 z-[-1] blur-2xl hidden dark:block"
        style={{
          maskImage:
            "linear-gradient(to bottom, transparent, white, transparent)",
          background:
            "repeating-linear-gradient(65deg, var(--color-blue-500), var(--color-blue-500) 12px, color-mix(in oklab, var(--color-blue-600) 30%, transparent) 20px, transparent 200px)",
        }}
      />
      <div
        className="absolute inset-0 z-[-1] blur-2xl dark:hidden"
        style={{
          maskImage:
            "linear-gradient(to bottom, transparent, white, transparent)",
          background:
            "repeating-linear-gradient(65deg, var(--color-purple-300), var(--color-purple-300) 12px, color-mix(in oklab, var(--color-blue-600) 30%, transparent) 20px, transparent 200px)",
        }}
      />
      <h1 className="mb-8 text-4xl font-medium md:hidden">ckan-devstaller</h1>
      <h1 className="mb-8 max-w-[800px] text-4xl font-medium max-md:hidden">
        <span className="text-5xl">
          ckan-devstaller{" "}
          <SailboatIcon className="inline-block w-10 h-10 pb-1" />
        </span>
        <br />
        Launch CKAN dev instances within minutes.
      </h1>
      <p className="mb-2 text-fd-muted-foreground md:max-w-[80%] md:text-xl">
        ckan-devstaller is a command-line tool to automate installing CKAN for
        development using ckan-compose on a new Ubuntu 22.04 instance.
      </p>
      <p className="mb-8 text-fd-muted-foreground md:max-w-[80%] md:text-sm">
        Provided by{" "}
        <Link className="text-fd-info" href="https://dathere.com">
          datHere
        </Link>
        .
      </p>
      <div className="inline-flex items-center gap-3 max-md:mx-auto mb-8">
        <Link
          href="/docs"
          className={cn(
            buttonVariants({ size: "lg", className: "rounded-full" }),
          )}
        >
          Get Started
        </Link>
        <Link
          href="https://github.com/dathere/ckan-devstaller"
          className={cn(
            buttonVariants({
              variant: "secondary",
              size: "lg",
              className: "rounded-full",
            }),
          )}
        >
          Source Code
        </Link>
      </div>
        <Cards>
          <Card
            icon={<ZapIcon />}
            href="/docs/quick-start"
            title="Quick start"
          >
            Get started with ckan-devstaller and install CKAN within minutes
          </Card>
          <Card icon={<BlocksIcon />} href="/docs/builder" title="Builder">
            Customize your installation with an interactive web GUI
          </Card>
          <Card
            icon={<HomeIcon />}
            href="/docs/reference/installation-architecture"
            title="Installation architecture"
          >
            Learn about where files are installed after running
            ckan-devstaller
          </Card>
          <Card
            icon={<GitMergeIcon />}
            href="https://github.com/dathere/ckan-devstaller"
            title="Source code"
          >
            View the source code of ckan-devstaller on GitHub
          </Card>
        </Cards>
      <PreviewImages />
    </div>
  );
}

function PreviewImages() {
  const [active, setActive] = useState(0);
  const previews = [
    {
      image: CkanDevstallerDemo,
      name: "Demo",
    },
  ];

  return (
    <div className="p-8 min-w-[800px] overflow-hidden xl:-mx-12 dark:[mask-image:linear-gradient(to_top,transparent,white_40px)]">
      <div className="absolute flex flex-row left-1/2 -translate-1/2 bottom-4 z-2 p-1 rounded-full bg-fd-card border shadow-xl dark:shadow-fd-background">
        {/* <div
          role="none"
          className="absolute bg-fd-primary rounded-full w-22 h-9 transition-transform z-[-1]"
          style={{
            transform: `translateX(calc(var(--spacing) * 22 * ${active}))`,
          }}
        /> */}
        {/* {previews.map((item, i) => (
          <button
            key={i}
            className={cn(previewButtonVariants({ active: active === i }))}
            onClick={() => setActive(i)}
          >
            {item.name}
          </button>
        ))} */}
      </div>
      {previews.map((item, i) => (
        <Image
          key={i}
          src={item.image}
          alt="preview"
          priority
          className={cn(
            "rounded-xl w-full select-none duration-1000 animate-in fade-in -mb-60 slide-in-from-bottom-12 lg:-mb-0",
            active !== i && "hidden",
          )}
        />
      ))}
    </div>
  );
}
