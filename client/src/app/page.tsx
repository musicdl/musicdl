import Image from "next/image";
import Link from "next/link";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      Home
      <Link href="/login" className="text-blue-700">
        {" "}
        Go To Login{" "}
      </Link>
    </main>
  );
}
