"use client";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { fetchMe } from "@/utils/api";

export default function MePage() {
  const router = useRouter();
  const [me, setMe] = useState<any>(null);

  useEffect(() => {
    const token = localStorage.getItem("jwt");
    if (!token) {
      router.push("/login");
      return;
    }
    fetchMe(token).then(setMe);
  }, [router]);

  if (!me) return <div className="text-center mt-20">Loading profile...</div>;

  return (
    <div className="max-w-md mx-auto mt-20 bg-white p-6 rounded shadow">
      <h1 className="text-xl font-semibold mb-2">My Profile</h1>
      <p><strong>ID:</strong> {me.id}</p>
      <p><strong>Username:</strong> {me.username}</p>
      <p><strong>Joined:</strong> {new Date(me.created_at).toLocaleString()}</p>
    </div>
  );
}
