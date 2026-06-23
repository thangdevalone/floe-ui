"use client";

import React, { useState, useMemo } from "react";
import * as LucideIcons from "lucide-react";
import iconsList from "@/lib/icons.json";

export function IconGallery() {
  const [search, setSearch] = useState("");
  const [copied, setCopied] = useState<string | null>(null);

  const filteredIcons = useMemo(() => {
    if (!search) return iconsList;
    return iconsList.filter((name) =>
      name.toLowerCase().includes(search.toLowerCase())
    );
  }, [search]);

  const handleCopy = (name: string) => {
    navigator.clipboard.writeText(`IconName::${name}`);
    setCopied(name);
    setTimeout(() => setCopied(null), 2000);
  };

  return (
    <div className="space-y-4 w-full">
      <div className="sticky top-0 z-10 bg-background/95 backdrop-blur py-4 flex flex-col gap-2">
        <input
          type="text"
          placeholder="Search icons..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="flex h-10 w-full max-w-md mx-auto rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        />
        <div className="flex justify-between items-center px-1">
          <p className="text-xs text-muted-foreground">Showing {filteredIcons.length} of {iconsList.length} icons</p>
          <p className="text-xs text-muted-foreground">Click on an icon to copy its Rust enum</p>
        </div>
      </div>

      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3 max-h-[600px] overflow-y-auto p-2 border border-border rounded-xl bg-card/30">
        {filteredIcons.map((name) => {
          const IconComponent = LucideIcons[name as keyof typeof LucideIcons] as React.ElementType;
          return (
            <div
              key={name}
              className="relative flex flex-col items-center justify-center p-3 gap-2 rounded-lg border border-border/50 bg-card hover:bg-accent hover:text-accent-foreground transition-all cursor-pointer group hover:shadow-sm"
              onClick={() => handleCopy(name)}
              title={`Click to copy IconName::${name}`}
            >
              {IconComponent ? (
                <IconComponent size={24} strokeWidth={1.5} className="opacity-80 group-hover:opacity-100 transition-opacity" />
              ) : (
                <div className="w-6 h-6 bg-muted rounded" />
              )}
              <span className="text-[10px] font-mono text-center truncate w-full text-muted-foreground group-hover:text-foreground">
                {name}
              </span>
              
              {copied === name && (
                <div className="absolute inset-0 bg-primary/90 text-primary-foreground rounded-lg flex items-center justify-center text-xs font-medium animate-in fade-in zoom-in duration-200">
                  Copied!
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
