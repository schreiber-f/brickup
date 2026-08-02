import { Card } from "@/components/ui/card";

export function Dashboard() {
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold">Dashboard</h1>

        <p className="text-muted-foreground">Willkommen zurück</p>
      </div>

      <section>
        <h2 className="mb-4 text-xl font-semibold">Meine Sets</h2>

        <div
          className="
            grid
            grid-cols-2
            xl:grid-cols-4
            gap-4
          "
        >
          <Card className="h-48">Test Set</Card>

          <Card className="h-48">Test Set</Card>
        </div>
      </section>
    </div>
  );
}
