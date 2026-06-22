import { cn } from "@/lib/utils";

export interface ApiProp {
  name: string;
  type: string;
  default?: string;
  description: string;
}

interface ApiTableProps {
  title?: string;
  props: ApiProp[];
  className?: string;
}

export function ApiTable({ title, props, className }: ApiTableProps) {
  return (
    <div className={cn("space-y-3", className)}>
      {title && <h4 className="text-sm font-semibold text-foreground">{title}</h4>}
      <div className="rounded-xl border border-border overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-border bg-muted/30">
              <th className="text-left px-4 py-3 font-medium text-muted-foreground">Name</th>
              <th className="text-left px-4 py-3 font-medium text-muted-foreground">Type</th>
              <th className="text-left px-4 py-3 font-medium text-muted-foreground hidden md:table-cell">Default</th>
              <th className="text-left px-4 py-3 font-medium text-muted-foreground">Description</th>
            </tr>
          </thead>
          <tbody>
            {props.map((prop) => (
              <tr
                key={prop.name}
                className={cn(
                  "border-b border-border/50 last:border-0 transition-colors hover:bg-muted/10",
                )}
              >
                <td className="px-4 py-3 font-mono text-[13px] text-violet-400">{prop.name}</td>
                <td className="px-4 py-3 font-mono text-[13px] text-cyan-400">{prop.type}</td>
                <td className="px-4 py-3 font-mono text-[13px] text-zinc-400 hidden md:table-cell">
                  {prop.default || "—"}
                </td>
                <td className="px-4 py-3 text-muted-foreground">{prop.description}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
