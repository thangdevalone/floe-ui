"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import { getAllComponents } from "@/lib/component-data";

const gettingStarted = [
  { title: "Introduction", href: "/docs" },
  { title: "Installation", href: "/docs/installation" },
  { title: "Theming", href: "/docs/theming" },
];

const componentItems = getAllComponents()
  .sort((a, b) => a.name.localeCompare(b.name))
  .map((c) => ({ title: c.name, href: `/docs/components/${c.slug}` }));

const sidebarNavItems = [
  { title: "Getting Started", items: gettingStarted },
  {
    title: "Components",
    items: [{ title: "All Components", href: "/docs/components" }, ...componentItems],
  },
];

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const pathname = usePathname();

  return (
    <div className="container mx-auto max-w-screen-2xl flex-1 items-start md:grid md:grid-cols-[220px_minmax(0,1fr)] md:gap-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:gap-10 px-4 md:px-8 py-8">
      <aside className="fixed top-14 z-30 -ml-2 hidden h-[calc(100vh-3.5rem)] w-full shrink-0 md:sticky md:block overflow-y-auto scrollbar-thin">
        <div className="h-full py-6 pr-6 lg:py-8">
          <div className="w-full">
            {sidebarNavItems.map((group, index) => (
              <div key={index} className="pb-4">
                <h4 className="mb-1 rounded-md px-2 py-1 text-sm font-semibold">
                  {group.title}
                </h4>
                <div className="grid grid-flow-row auto-rows-max text-sm">
                  {group.items.map((item, itemIndex) => {
                    const isActive = pathname === item.href;
                    return (
                      <Link
                        key={itemIndex}
                        href={item.href}
                        className={cn(
                          "group flex w-full items-center rounded-md border border-transparent px-2 py-1 transition-colors",
                          isActive
                            ? "font-medium text-foreground bg-muted/50"
                            : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
                        )}
                      >
                        {item.title}
                      </Link>
                    );
                  })}
                </div>
              </div>
            ))}
          </div>
        </div>
      </aside>
      <main className="relative py-6 lg:gap-10 lg:py-8 xl:grid xl:grid-cols-[1fr_300px]">
        <div className="mx-auto w-full min-w-0">{children}</div>
      </main>
    </div>
  );
}
