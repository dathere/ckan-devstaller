/** biome-ignore-all lint/suspicious/noArrayIndexKey: Would need to look into this trivial issue */
"use client";

import { CodeBlock } from "fumadocs-ui/components/codeblock";
import defaultMdxComponents from "fumadocs-ui/mdx";
import { cn } from "fumadocs-ui/utils/cn";
import {
  BlocksIcon,
  GitMergeIcon,
  HomeIcon,
  SailboatIcon,
  TerminalIcon,
  Trash2Icon,
  ZapIcon,
} from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { type HTMLProps, type ReactNode, useState } from "react";
import { Pre } from "@/components/codeblock";
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
            <Why />
          </div>
        </div>
        <hr className="mt-12 mb-4" />
        <footer className="flex flex-col bg-brand-secondary pb-12 text-brand-secondary-foreground rounded-2xl">
          <p className="mb-1 text-xl font-semibold">ckan-devstaller</p>
          <p className="text-xs">
            Provided by{" "}
            <a
              href="https://dathere.com"
              target="_blank"
              className="font-medium text-blue-400"
              rel="noopener"
            >
              datHere
            </a>
            .{" "}
            <a
              href="https://dathere.com/privacy-policy/"
              target="_blank"
              className="font-medium text-blue-400"
              rel="noopener"
            >
              Privacy Policy
            </a>
            .
          </p>
        </footer>
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
        <Card icon={<ZapIcon />} href="/docs" title="Quick start">
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
          Learn about where files are installed after running ckan-devstaller
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
    <div className="p-8 min-w-[600px] md:min-w-[800px] overflow-hidden xl:-mx-12 dark:[mask-image:linear-gradient(to_top,transparent,white_40px)]">
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
            "rounded-xl w-full select-none duration-1000 animate-in fade-in md:-mb-60 slide-in-from-bottom-12 lg:-mb-0",
            active !== i && "hidden",
          )}
        />
      ))}
    </div>
  );
}

function Why() {
  return (
    <div className="relative overflow-hidden border-x border-t p-2">
      <WhyInteractive
        codeblockInstall={
          <CodeBlock lang="bash">
            <Pre className="text-wrap pl-4">./ckan-devstaller</Pre>
          </CodeBlock>
        }
        codeblockUninstall={
          <CodeBlock lang="bash">
            <Pre className="text-wrap pl-4">./ckan-devstaller uninstall</Pre>
          </CodeBlock>
        }
      />
    </div>
  );
}

export function WhyInteractive(props: {
  codeblockInstall: ReactNode;
  codeblockUninstall: ReactNode;
}) {
  const [active, setActive] = useState(0);
  const items = [
    [
      <ZapIcon className="w-4 h-4 inline-block" key={0} />,
      "Install CKAN within minutes",
    ],
    [
      <BlocksIcon className="w-4 h-4 inline-block" key={1} />,
      "Customize your installation",
    ],
    [
      <TerminalIcon className="w-4 h-4 inline-block" key={2} />,
      "Designed for developers",
    ],
    [
      <Trash2Icon className="w-4 h-4 inline-block" key={3} />,
      "Uninstall with ease",
    ],
  ];

  return (
    <div
      id="why-interactive"
      className="flex flex-col-reverse gap-3 md:flex-row md:min-h-[200px]"
    >
      <div className="flex flex-col">
        {items.map((item, i) => (
          <button
            key={item[1] as string}
            ref={(element) => {
              if (!element || i !== active) return;
            }}
            type="button"
            className={cn(
              "transition-colors text-nowrap border border-transparent rounded-lg px-3 py-2.5 text-start text-sm text-fd-muted-foreground font-medium",
              i === active
                ? "text-fd-primary bg-fd-primary/10 border-fd-primary/10"
                : "hover:text-fd-accent-foreground/80 cursor-pointer",
            )}
            onClick={() => {
              setActive(i);
            }}
          >
            {item[0]} {item[1]}
          </button>
        ))}
      </div>
      <style>
        {`
        @keyframes why-interactive-x {
          from {
            width: 0px;
          }
          
          to {
            width: 100%;
          }
        }`}
      </style>

      <div className="flex-1 p-4 border border-fd-primary/10 bg-fd-card/40 rounded-lg shadow-lg">
        {active === 0 ? (
          <WhyPanel>
            <h3>
              <ZapIcon className="w-4 h-4 inline-block mr-1 mb-1" />
              Install CKAN within minutes.
            </h3>
            <p>
              One of the primary goals of ckan-devstaller is to ease
              installation of CKAN for development. Built with Rust for speed
              and streamlining installation with{" "}
              <a href="https://github.com/tino097/ckan-compose/tree/ckan-devstaller">
                ckan-compose
              </a>
              , ckan-devstaller improves installation speeds{" "}
              <strong>from hours/days to just minutes</strong> depending on your
              download speed.
            </p>
            <div className="flex gap-2">
              <Link href="/docs" className={cn(buttonVariants(), "not-prose")}>
                Get started
              </Link>
              <Link
                href="/docs/builder"
                className={cn(buttonVariants(), "not-prose")}
              >
                Customize your installation
              </Link>
            </div>
          </WhyPanel>
        ) : null}
        {active === 1 ? (
          <WhyPanel>
            <h3>
              <BlocksIcon className="w-4 h-4 inline-block mr-1 mb-1" />
              Customize your installation with the Builder.
            </h3>
            <p>
              Try out the interactive web GUI for customizing your CKAN
              installation. You can select:
            </p>
            <ul>
              <li>Presets</li>
              <li>CKAN version</li>
              <li>Extensions</li>
              <li>Features</li>
            </ul>
            <p>
              Then you can copy the provided ckan-devstaller command to run your
              selected configuration.
            </p>
            <div className="mt-4 flex flex-row items-center gap-1.5 not-prose">
              <Link href="/docs/builder" className={cn(buttonVariants())}>
                Try out the Builder
              </Link>
            </div>
          </WhyPanel>
        ) : null}
        {active === 2 ? (
          <WhyPanel>
            <h3>
              <TerminalIcon className="w-4 h-4 inline-block mr-1 mb-1" />
              Designed for developers.
            </h3>
            <p>
              We've kept development use cases in mind while developing
              ckan-devstaller, such as:
            </p>
            <ul>
              <li>Trying out a new version of CKAN</li>
              <li>Developing CKAN extensions and themes</li>
            </ul>
            <div className="flex gap-2">
              <Link
                href="/docs/reference/installation-architecture"
                className={cn(buttonVariants(), "not-prose")}
              >
                View the installation architecture
              </Link>
              <Link
                href="https://github.com/dathere/ckan-devstaller"
                className={cn(buttonVariants({ variant: "ghost" }))}
              >
                Source code
              </Link>
            </div>
          </WhyPanel>
        ) : null}
        {active === 3 ? (
          <WhyPanel>
            <h3>
              <Trash2Icon className="w-4 h-4 inline-block mr-1 mb-1" />
              Uninstall CKAN with ease.
            </h3>
            <p>
              After you've installed CKAN with ckan-devstaller, you can
              uninstall CKAN with ease. This allows for quickly re-installing
              CKAN for a different use case.
            </p>
            {props.codeblockUninstall}
            <Link
              href="/docs/tutorials/uninstall-ckan"
              className={cn(buttonVariants(), "not-prose")}
            >
              Learn more about uninstalling
            </Link>
          </WhyPanel>
        ) : null}
      </div>
    </div>
  );
}

function WhyPanel(props: HTMLProps<HTMLDivElement>) {
  return (
    <div
      {...props}
      className={cn(
        "duration-700 animate-in fade-in text-sm prose",
        props.className,
      )}
    >
      {props.children}
    </div>
  );
}
