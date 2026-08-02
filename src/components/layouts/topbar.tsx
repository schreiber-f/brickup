import { SidebarTrigger } from "@/components/ui/sidebar";
import { Input } from "@/components/ui/input";

export function Topbar() {
  return (
    <header
      className="
        h-16
        border-b
        border-border
        flex
        items-center
        gap-4
        px-4
      "
    >
      <SidebarTrigger size="lg" />

      <Input
        placeholder="LEGO Set suchen..."
        className="
          max-w-xl
          bg-card
        "
      />
    </header>
  );
}
