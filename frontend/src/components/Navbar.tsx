"use client";
import { useRouter } from "next/navigation";

export default function Navbar() {
  const router = useRouter();

  const logout = () => {
    localStorage.removeItem("jwt");
    router.push("/login");
  };

  return (
    <nav className="bg-white shadow-md px-6 py-4 flex justify-between items-center">
      <div
        className="text-lg font-bold text-gray-800 cursor-pointer"
        onClick={() => router.push("/chat")}
      >
        Rusty Chat
      </div>
      <div className="flex gap-4">
        <button
          onClick={() => router.push("/me")}
          className="text-sm px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
        >
          My Profile
        </button>
        <button
          onClick={logout}
          className="text-sm px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Logout
        </button>
      </div>
    </nav>
  );
}