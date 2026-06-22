"use client";

import { notFound } from "next/navigation";
import { use } from "react";
import { ChevronRight } from "lucide-react";
import Link from "next/link";
import { componentsData } from "@/lib/component-data";
import { CodeBlock } from "@/components/code-block";
import { ApiTable } from "@/components/api-table";

export default function ComponentPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const resolvedParams = use(params);
  const slug = resolvedParams.slug;
  const data = componentsData[slug];

  if (!data) {
    notFound();
  }

  return (
    <div className="space-y-10">
      {/* Breadcrumbs */}
      <div className="flex items-center space-x-1 text-sm text-muted-foreground">
        <Link href="/docs" className="hover:text-foreground transition-colors">Docs</Link>
        <ChevronRight className="h-3.5 w-3.5" />
        <Link href="/docs/components" className="hover:text-foreground transition-colors">Components</Link>
        <ChevronRight className="h-3.5 w-3.5" />
        <span className="font-medium text-foreground">{data.name}</span>
      </div>

      {/* Header */}
      <div className="space-y-2">
        <div className="flex items-center gap-3">
          <h1 className="scroll-m-20 text-4xl font-bold tracking-tight">{data.name}</h1>
          <span className="text-xs font-medium text-muted-foreground bg-muted/50 rounded-full px-2.5 py-1 mt-1">
            {data.category}
          </span>
        </div>
        <p className="text-lg text-muted-foreground">{data.description}</p>
      </div>

      {/* Preview */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Preview</h2>
        <div className="min-h-[200px] flex items-center justify-center rounded-xl border border-border p-8 md:p-10 bg-background/50">
          {data.preview}
        </div>
      </section>

      {/* Variants */}
      {data.variants && data.variants.length > 0 && (
        <section className="space-y-4">
          <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Variants</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {data.variants.map((variant) => (
              <div
                key={variant.label}
                className="flex flex-col items-center gap-4 rounded-xl border border-border/50 p-6 bg-card/30 hover:bg-card/60 transition-colors"
              >
                <div className="flex items-center justify-center min-h-[60px]">
                  {variant.element}
                </div>
                <span className="text-xs font-medium text-muted-foreground">{variant.label}</span>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Usage Code */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Usage</h2>
        <CodeBlock code={data.code} filename={`${slug}.rs`} />
      </section>

      {/* API Reference */}
      <section className="space-y-6">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">API Reference</h2>
        <ApiTable title="Props / Parameters" props={data.apiProps} />

        {/* Style Functions */}
        {data.styleFunctions.length > 0 && (
          <div className="space-y-3">
            <h4 className="text-sm font-semibold text-foreground">Style Functions</h4>
            <div className="rounded-xl border border-border overflow-hidden">
              <div className="divide-y divide-border/50">
                {data.styleFunctions.map((fn) => (
                  <div key={fn} className="px-4 py-2.5 flex items-center gap-2 hover:bg-muted/10 transition-colors">
                    <span className="font-mono text-[13px] text-emerald-400">{fn}</span>
                  </div>
                ))}
              </div>
            </div>
            <p className="text-xs text-muted-foreground">
              Style functions return closures compatible with Iced&apos;s <code className="text-violet-400">.style()</code> method.
            </p>
          </div>
        )}

        {/* Builder Functions */}
        {data.builderFunctions.length > 0 && (
          <div className="space-y-3">
            <h4 className="text-sm font-semibold text-foreground">Builder Functions</h4>
            <div className="rounded-xl border border-border overflow-hidden">
              <div className="divide-y divide-border/50">
                {data.builderFunctions.map((fn) => (
                  <div key={fn} className="px-4 py-2.5 flex items-center gap-2 hover:bg-muted/10 transition-colors">
                    <span className="font-mono text-[13px] text-yellow-300">{fn}</span>
                  </div>
                ))}
              </div>
            </div>
            <p className="text-xs text-muted-foreground">
              Builder functions return pre-configured Iced widgets ready for use.
            </p>
          </div>
        )}
      </section>

      {/* Source */}
      <section className="pt-4 border-t border-border">
        <p className="text-sm text-muted-foreground">
          Source: <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">floe-ui/src/components/{slug === "toggle-group" ? "toggle_group" : slug}.rs</code>
        </p>
      </section>
    </div>
  );
}
