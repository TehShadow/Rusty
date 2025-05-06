"use client";
import { useEffect, useRef, useState } from "react";
import { useRouter } from "next/navigation";

export default function ChatPage() {
  const router = useRouter();
  const socket = useRef<WebSocket | null>(null);
  const [messages, setMessages] = useState<string[]>([]);
  const [input, setInput] = useState("");

  useEffect(() => {
    const token = localStorage.getItem("jwt");
    if (!token) {
      router.push("/login");
      return;
    }
    socket.current = new WebSocket(`ws://localhost:3001/ws?token=${token}`);

    socket.current.onmessage = (event) => {
      setMessages((msgs) => [...msgs, event.data]);
    };

    socket.current.onclose = () => {
      setMessages((msgs) => [...msgs, "[Disconnected]"]);
    };

    return () => {
      socket.current?.close();
    };
  }, [router]);

  const sendMessage = () => {
    if (socket.current && input.trim()) {
      socket.current.send(input);
      setInput("");
    }
  };

  return (
    <div className="max-w-xl mx-auto">
      <h1 className="text-2xl mb-4">Chat</h1>
      <div className="border h-64 overflow-y-scroll p-2 mb-4 bg-white">
        {messages.map((msg, i) => (
          <div key={i} className="text-sm py-1 border-b">{msg}</div>
        ))}
      </div>
      <div className="flex gap-2">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="flex-1 p-2 border"
          placeholder="Type a message..."
        />
        <button className="bg-blue-600 text-white px-4 py-2" onClick={sendMessage}>
          Send
        </button>
      </div>
    </div>
  );
}