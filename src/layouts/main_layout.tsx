import { Topbar } from "@/components/layouts/topbar";
import { Dashboard } from "@/pages/dashboard";
import { AppSidebar } from "@/components/layouts/sidebar";

import { SidebarProvider } from "@/components/ui/sidebar";

export function MainLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />

      <main
        className="
        flex-1
        min-h-screen
        bg-background
      "
      >
        <Topbar />

        <div className="p-6">
          <Dashboard />
        </div>
      </main>
    </SidebarProvider>
  );
}
