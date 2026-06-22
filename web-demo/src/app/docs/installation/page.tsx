import { CodeBlock } from "@/components/code-block";

export default function InstallationPage() {
  return (
    <div className="space-y-10">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-bold tracking-tight">Installation</h1>
        <p className="text-lg text-muted-foreground">
          How to get Floe UI into your Iced project.
        </p>
      </div>

      {/* Philosophy */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Philosophy</h2>
        <p className="leading-7">
          Floe UI is <strong>not a traditional crate dependency</strong>. Instead, you copy the source
          code directly into your project, just like{" "}
          <a href="https://ui.shadcn.com" target="_blank" className="font-medium underline underline-offset-4">
            shadcn/ui
          </a>{" "}
          does for React. This gives you full ownership and complete customization freedom.
        </p>
      </section>

      {/* Steps */}
      <section className="space-y-6">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Setup</h2>

        <div className="space-y-4">
          <div className="flex items-start gap-4">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted/50 text-sm font-semibold">1</span>
            <div className="space-y-2 flex-1">
              <h3 className="font-semibold">Add Iced to your project</h3>
              <p className="text-sm text-muted-foreground">Make sure you have Iced as a dependency in your Cargo.toml.</p>
              <CodeBlock
                code={`[dependencies]
iced = { version = "0.14", features = ["advanced"] }`}
                filename="Cargo.toml"
                language="toml"
              />
            </div>
          </div>

          <div className="flex items-start gap-4">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted/50 text-sm font-semibold">2</span>
            <div className="space-y-2 flex-1">
              <h3 className="font-semibold">Install the Floe CLI</h3>
              <p className="text-sm text-muted-foreground">
                Install the official Floe CLI globally to manage your components easily.
              </p>
              <CodeBlock
                code={`cargo install floe-ui`}
                filename="terminal"
                language="bash"
              />
            </div>
          </div>

          <div className="flex items-start gap-4">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted/50 text-sm font-semibold">3</span>
            <div className="space-y-2 flex-1">
              <h3 className="font-semibold">Initialize Floe in your project</h3>
              <p className="text-sm text-muted-foreground">
                Run the init command inside your Iced project. This will set up the <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">src/floe_ui</code> folder with the base theme system.
              </p>
              <CodeBlock
                code={`floe-ui init`}
                filename="terminal"
                language="bash"
              />
            </div>
          </div>

          <div className="flex items-start gap-4">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted/50 text-sm font-semibold">4</span>
            <div className="space-y-2 flex-1">
              <h3 className="font-semibold">Add components</h3>
              <p className="text-sm text-muted-foreground">
                Add only the components you need. The CLI will download the raw source code straight into your project.
              </p>
              <CodeBlock
                code={`floe-ui add button
floe-ui add input
floe-ui add card`}
                filename="terminal"
                language="bash"
              />
            </div>
          </div>

          <div className="flex items-start gap-4">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted/50 text-sm font-semibold">5</span>
            <div className="space-y-2 flex-1">
              <h3 className="font-semibold">Start using components</h3>
              <p className="text-sm text-muted-foreground">
                Import the prelude and start building your UI.
              </p>
              <CodeBlock
                code={`mod floe_ui; // Khai báo module đã được CLI tạo ra

use floe_ui::prelude::*;
use floe_ui::components::{button, card, input};

use iced::widget::column;
use iced::Element;

fn view(&self) -> Element<Message> {
    let theme = FloeTheme::zinc_dark();
    let tokens = &theme.tokens;

    let btn = button::primary("Click me", tokens)
        .on_press(Message::Clicked);

    let field = input::styled("Email…", &self.email, tokens)
        .on_input(Message::EmailChanged);

    let my_card = card::styled(
        column![btn, field].spacing(12),
        tokens,
    );

    my_card.into()
}`}
                filename="main.rs"
              />
            </div>
          </div>
        </div>
      </section>

      {/* Fonts */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Fonts (Optional)</h2>
        <p className="leading-7">
          Floe UI includes a bundled icon font (based on Lucide) for icon support. To enable icons,
          load the font files from the <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">floe-ui/fonts/</code> directory in your Iced application settings.
        </p>
      </section>
    </div>
  );
}
