"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { registerUser } from "@/utils/api";

export default function RegisterPage() {
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleRegister = async () => {
    const success = await registerUser(username, password);
    if (success) {
      router.push("/login");
    } else {
      setError("Registration failed");
    }
  };

  return (
    <div className="max-w-sm mx-auto">
      <h1 className="text-2xl mb-4">Register</h1>
      <input
        type="text"
        placeholder="Username"
        className="w-full mb-2 p-2 border"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <input
        type="password"
        placeholder="Password"
        className="w-full mb-4 p-2 border"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      {error && <div className="text-red-500 mb-2">{error}</div>}
      <button
        className="w-full bg-green-600 text-white p-2 rounded mb-2"
        onClick={handleRegister}
      >
        Register
      </button>
      <button
        className="w-full bg-gray-600 text-white p-2 rounded"
        onClick={() => router.push("/login")}
      >
        Back to Login
      </button>
    </div>
  );
}