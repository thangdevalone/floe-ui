"use client";

import { useState, useMemo } from "react";
import { motion } from "framer-motion";
import { cn } from "@/lib/utils";
import { Search } from "lucide-react";
import Link from "next/link";
import { getAllComponents, categories, type ComponentCategory } from "@/lib/component-data";

export default function ComponentsPage() {
  const [search, setSearch] = useState("");
  const [activeCategory, setActiveCategory] = useState<ComponentCategory | "All">("All");

  const allComponents = useMemo(() => getAllComponents(), []);

  const filtered = useMemo(() => {
    return allComponents.filter((c) => {
      const matchesSearch =
        !search ||
        c.name.toLowerCase().includes(search.toLowerCase()) ||
        c.description.toLowerCase().includes(search.toLowerCase());
      const matchesCategory = activeCategory === "All" || c.category === activeCategory;
      return matchesSearch && matchesCategory;
    });
  }, [allComponents, search, activeCategory]);

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-bold tracking-tight">Components</h1>
        <p className="text-lg text-muted-foreground">
          Browse all {allComponents.length} beautifully styled components available in Floe UI.
        </p>
      </div>

      {/* Search + Filter */}
      <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4">
        <div className="relative w-full max-w-sm">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <input
            id="component-search"
            placeholder="Search components..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 pl-8 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
          />
        </div>

        <div className="flex flex-wrap gap-1.5">
          {(["All", ...categories] as const).map((cat) => (
            <button
              key={cat}
              onClick={() => setActiveCategory(cat)}
              className={cn(
                "inline-flex items-center rounded-full px-3 py-1 text-xs font-medium transition-colors",
                activeCategory === cat
                  ? "bg-primary text-primary-foreground"
                  : "bg-muted/50 text-muted-foreground hover:bg-muted hover:text-foreground"
              )}
            >
              {cat}
            </button>
          ))}
        </div>
      </div>

      {/* Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
        {filtered.map((comp, i) => (
          <Link key={comp.slug} href={`/docs/components/${comp.slug}`}>
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: i * 0.03 }}
              className="group flex flex-col rounded-xl border border-border bg-card text-card-foreground shadow-sm overflow-hidden h-full hover:border-primary/50 hover:shadow-md transition-all cursor-pointer"
            >
              <div className="flex-1 flex items-center justify-center p-6 bg-muted/10 min-h-[180px]">
                {comp.preview}
              </div>
              <div className="p-5 border-t border-border group-hover:bg-muted/10 transition-colors">
                <div className="flex items-center justify-between mb-1.5">
                  <h3 className="font-semibold leading-none tracking-tight">{comp.name}</h3>
                  <span className="text-[10px] font-medium text-muted-foreground bg-muted/50 rounded-full px-2 py-0.5">
                    {comp.category}
                  </span>
                </div>
                <p className="text-sm text-muted-foreground line-clamp-2">
                  {comp.description}
                </p>
              </div>
            </motion.div>
          </Link>
        ))}
      </div>

      {filtered.length === 0 && (
        <div className="text-center py-16">
          <p className="text-muted-foreground">No components found matching &ldquo;{search}&rdquo;.</p>
        </div>
      )}
    </div>
  );
}
