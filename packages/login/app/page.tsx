import { redirect } from "next/navigation";

export default function Home() {
  redirect("/login/1");
  return null;
}
