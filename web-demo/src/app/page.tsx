"use client";

import { motion } from "framer-motion";
import Link from "next/link";
import { ArrowRight, Code2, Paintbrush, Layers } from "lucide-react";
import { cn } from "@/lib/utils";

export default function Home() {
  return (
    <div className="flex flex-col items-center w-full min-h-screen">
      {/* Background gradients */}
      <div className="fixed inset-0 z-[-1] bg-background">
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_80%_80%_at_50%_-20%,rgba(120,119,198,0.15),rgba(255,255,255,0))]" />
      </div>

      {/* Hero Section */}
      <section className="w-full flex flex-col items-center justify-center pt-32 pb-20 px-4 text-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="max-w-[800px] flex flex-col items-center space-y-8"
        >
          <div className="inline-flex items-center rounded-full border border-border px-3 py-1 text-sm text-muted-foreground backdrop-blur-sm">
            <span className="flex h-2 w-2 rounded-full bg-primary mr-2" />
            Introducing Floe UI for Iced
          </div>
          
          <h1 className="text-4xl md:text-6xl lg:text-7xl font-bold tracking-tight text-foreground max-w-4xl text-balance">
            Build beautiful <span className="text-transparent bg-clip-text bg-gradient-to-r from-zinc-200 to-zinc-600">Iced Apps</span> faster.
          </h1>
          
          <p className="text-xl text-muted-foreground max-w-[600px] leading-relaxed">
            Beautifully styled, token-driven components that you own and can customize freely — built for the Rust Iced GUI framework.
          </p>

          <div className="flex flex-col sm:flex-row items-center gap-4 pt-4">
            <Link
              href="/docs"
              className={cn(
                "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
                "bg-primary text-primary-foreground shadow hover:bg-primary/90 h-10 px-8 py-2 rounded-md"
              )}
            >
              Get Started
              <ArrowRight className="ml-2 h-4 w-4" />
            </Link>
            <Link
              href="https://github.com/thang/floe-ui"
              target="_blank"
              className={cn(
                "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
                "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-10 px-8 py-2 rounded-md"
              )}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="mr-2 h-4 w-4"
              >
                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
                <path d="M9 18c-4.51 2-5-2-7-2" />
              </svg>
              GitHub
            </Link>
          </div>
        </motion.div>
      </section>

      {/* Features Grid */}
      <section className="w-full max-w-[1200px] px-4 py-20">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <FeatureCard 
            icon={<Paintbrush className="h-6 w-6 text-primary" />}
            title="Token-driven Design"
            description="Built from the ground up using a comprehensive design token system. Easily customize colors, radius, shadows, and more across your entire app."
            delay={0.1}
          />
          <FeatureCard 
            icon={<Code2 className="h-6 w-6 text-primary" />}
            title="You Own The Code"
            description="Inspired by shadcn/ui. The components are yours. Copy and paste them into your project and customize them to your exact needs."
            delay={0.2}
          />
          <FeatureCard 
            icon={<Layers className="h-6 w-6 text-primary" />}
            title="Rich Components"
            description="Includes complex components out of the box like Dropdowns, Data Tables, Pagination, Tabs, Toggles, and much more."
            delay={0.3}
          />
        </div>
      </section>

      {/* Visual Showcase (Mockup) */}
      <section className="w-full max-w-[1200px] px-4 py-20 flex flex-col items-center">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold tracking-tight mb-4">Crafted with precision</h2>
          <p className="text-muted-foreground">Every component is meticulously designed to provide the best user experience.</p>
        </div>

        <motion.div 
          initial={{ opacity: 0, y: 40 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="w-full max-w-[800px] rounded-xl border border-border/50 bg-background/50 backdrop-blur-xl p-8 shadow-2xl overflow-hidden relative"
        >
          {/* Mock Window Header */}
          <div className="flex items-center gap-2 mb-8 border-b border-border/40 pb-4">
            <div className="flex gap-1.5">
              <div className="w-3 h-3 rounded-full bg-red-500/80" />
              <div className="w-3 h-3 rounded-full bg-yellow-500/80" />
              <div className="w-3 h-3 rounded-full bg-green-500/80" />
            </div>
            <div className="mx-auto text-xs font-medium text-muted-foreground">floe-ui-gallery</div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="flex flex-col gap-6">
              {/* Mock Buttons */}
              <div className="space-y-3">
                <h3 className="text-sm font-medium">Buttons</h3>
                <div className="flex flex-wrap gap-2">
                  <button className="bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-4 py-2 rounded-md text-sm font-medium transition-colors">Primary</button>
                  <button className="bg-secondary text-secondary-foreground hover:bg-secondary/80 h-9 px-4 py-2 rounded-md text-sm font-medium transition-colors">Secondary</button>
                  <button className="border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 rounded-md text-sm font-medium transition-colors">Outline</button>
                </div>
              </div>

              {/* Mock Inputs */}
              <div className="space-y-3">
                <h3 className="text-sm font-medium">Inputs</h3>
                <input 
                  type="text" 
                  placeholder="Enter your email..." 
                  className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                  disabled
                />
              </div>
            </div>

            <div className="flex flex-col gap-6">
               {/* Mock Dropdown */}
               <div className="space-y-3">
                <h3 className="text-sm font-medium">Dropdown Menu</h3>
                <div className="relative inline-block text-left w-full max-w-[220px]">
                  <div className="w-full rounded-md border border-border bg-popover text-popover-foreground shadow-md p-1">
                    <div className="px-2 py-1.5 text-xs font-medium text-muted-foreground">My Account</div>
                    <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">
                      Profile
                      <span className="ml-auto text-xs tracking-widest text-muted-foreground">⇧⌘P</span>
                    </button>
                    <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">
                      Billing
                      <span className="ml-auto text-xs tracking-widest text-muted-foreground">⌘B</span>
                    </button>
                    <div className="my-1 h-px bg-muted" />
                    <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">
                      Log out
                      <span className="ml-auto text-xs tracking-widest text-muted-foreground">⇧⌘Q</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </motion.div>
      </section>
      
      <footer className="w-full border-t border-border mt-auto py-8 text-center text-sm text-muted-foreground">
        Built by Thang. Inspired by shadcn/ui. Designed for Iced.
      </footer>
    </div>
  );
}

function FeatureCard({ icon, title, description, delay }: { icon: React.ReactNode, title: string, description: string, delay: number }) {
  return (
    <motion.div 
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      transition={{ duration: 0.5, delay }}
      className="flex flex-col gap-4 rounded-xl border border-border/50 bg-card p-6 shadow-sm hover:shadow-md transition-shadow"
    >
      <div className="h-12 w-12 rounded-lg bg-primary/10 flex items-center justify-center">
        {icon}
      </div>
      <h3 className="text-xl font-semibold tracking-tight">{title}</h3>
      <p className="text-muted-foreground leading-relaxed">{description}</p>
    </motion.div>
  );
}
