import InspectorShell from "./components/inspector-shell";
import { loadInspectorData } from "../src/lib/server-data";

export const dynamic = "force-dynamic";

export default async function Page() {
  const data = await loadInspectorData();
  return <InspectorShell initialData={data} />;
}
