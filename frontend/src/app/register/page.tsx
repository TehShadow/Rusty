"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { registerUser } from "@/utils/api";

export default function RegisterPage() {
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [success, setSuccess] = useState(false);

  const handleRegister = async () => {
    const result = await registerUser(username, password);
    if (result) {
      setSuccess(true);
      setTimeout(() => router.push("/login"), 1500);
    } else {
      setError("Registration failed");
    }
  };

  return (
    <div className="max-w-md mx-auto mt-20 p-6 bg-white rounded shadow">
      <h1 className="text-2xl mb-4 text-center">Register</h1>
      <input
        className="w-full mb-3 p-2 border rounded"
        placeholder="Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <input
        className="w-full mb-3 p-2 border rounded"
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      {error && <div className="text-red-500 mb-2">{error}</div>}
      {success && <div className="text-green-600 mb-2">Registered! Redirecting...</div>}
      <button
        onClick={handleRegister}
        className="w-full bg-green-600 hover:bg-green-700 text-white py-2 rounded"
      >
        Register
      </button>
    </div>
  );
}