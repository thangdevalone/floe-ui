import Link from "next/link";
import { ArrowRight } from "lucide-react";
import { cn } from "@/lib/utils";

export default function DocsPage() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-bold tracking-tight">Introduction</h1>
        <p className="text-lg text-muted-foreground">
          Beautifully styled, token-driven components for the Iced GUI framework.
        </p>
      </div>

      <div className="space-y-4">
        <p className="leading-7">
          Floe UI is a collection of reusable components built natively in Rust for the 
          <a href="https://github.com/iced-rs/iced" target="_blank" className="font-medium underline underline-offset-4 ml-1">Iced framework</a>. 
          It is highly inspired by the popular React library <strong>shadcn/ui</strong>.
        </p>
        <p className="leading-7">
          <strong>This is NOT a component library.</strong> It&apos;s a collection of components 
          that you can copy and paste directly into your Iced apps.
        </p>
      </div>

      <div className="space-y-4 pt-8">
        <h2 className="scroll-m-20 border-b pb-2 text-2xl font-semibold tracking-tight">
          What do you mean by &quot;not a component library&quot;?
        </h2>
        <p className="leading-7">
          I mean you do not install it as a dependency from Crates.io (though we might provide a core traits crate in the future).
          <br />
          You own the code. You can copy the source code of the component you need and customize it however you want.
        </p>
      </div>

      <div className="pt-8">
        <Link
          href="/docs/components"
          className={cn(
            "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
            "bg-primary text-primary-foreground shadow hover:bg-primary/90 h-10 px-8 py-2 rounded-md"
          )}
        >
          View Components
          <ArrowRight className="ml-2 h-4 w-4" />
        </Link>
      </div>
    </div>
  );
}
