"use client";

import { useState, useCallback } from "react";
import { Check, Copy } from "lucide-react";
import { cn } from "@/lib/utils";

// Simple Rust-like syntax highlighting via regex spans
function highlightRust(code: string): React.ReactNode[] {
  const lines = code.split("\n");
  return lines.map((line, i) => {
    const highlighted = line
      // Comments
      .replace(/(\/\/.*)/g, '<span class="text-zinc-500">$1</span>')
      // Strings
      .replace(/("(?:[^"\\]|\\.)*")/g, '<span class="text-emerald-400">$1</span>')
      // Keywords
      .replace(
        /\b(use|pub|fn|let|mut|self|Self|impl|struct|enum|match|if|else|for|in|return|where|move|async|await|mod|crate|super|true|false|None|Some|Ok|Err)\b/g,
        '<span class="text-violet-400">$1</span>'
      )
      // Types
      .replace(
        /\b(DesignTokens|Message|Theme|Element|Button|Container|Row|Column|Style|Status|Color|Background|Border|Shadow|String|Option|Vec|f32|bool|usize|str)\b/g,
        '<span class="text-cyan-400">$1</span>'
      )
      // Functions/methods after ::
      .replace(
        /::(\w+)/g,
        '::<span class="text-yellow-300">$1</span>'
      )
      // Macros
      .replace(
        /\b(\w+!)\(/g,
        '<span class="text-orange-400">$1</span>('
      );
    return (
      <span key={i}>
        <span dangerouslySetInnerHTML={{ __html: highlighted }} />
        {i < lines.length - 1 ? "\n" : ""}
      </span>
    );
  });
}

interface CodeBlockProps {
  code: string;
  filename?: string;
  language?: string;
  className?: string;
}

export function CodeBlock({ code, filename = "example.rs", language = "rust", className }: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = useCallback(() => {
    navigator.clipboard.writeText(code).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  }, [code]);

  return (
    <div className={cn("rounded-xl border border-border bg-zinc-950 overflow-hidden", className)}>
      <div className="px-4 py-2.5 border-b border-white/10 bg-zinc-900/50 flex items-center justify-between">
        <span className="text-xs font-medium text-zinc-400 font-mono">{filename}</span>
        <button
          onClick={handleCopy}
          className="inline-flex items-center gap-1.5 text-xs text-zinc-500 hover:text-zinc-300 transition-colors px-2 py-1 rounded-md hover:bg-white/5"
          aria-label="Copy code"
        >
          {copied ? (
            <>
              <Check className="h-3.5 w-3.5 text-emerald-400" />
              <span className="text-emerald-400">Copied</span>
            </>
          ) : (
            <>
              <Copy className="h-3.5 w-3.5" />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>
      <pre className="p-4 overflow-x-auto text-[13px] leading-relaxed">
        <code className="text-zinc-50 font-mono">
          {language === "rust" ? highlightRust(code) : code}
        </code>
      </pre>
    </div>
  );
}
