"use client";
import { useEffect, useRef } from "react";

export default function useChatSocket(room: any, setMessages: (msg: any[]) => void) {
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const token = localStorage.getItem("jwt");
    if (!room || !token) return;

    if (socketRef.current) socketRef.current.close();

    const ws = new WebSocket(
      `ws://localhost:3001/ws?room_id=${room.id}&token=${token}`
    );

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === "message") {
        setMessages((prev) => [...prev, data.payload]);
      }
    };

    socketRef.current = ws;
    return () => ws.close();
  }, [room]);

  const sendMessageThroughSocket = (text: string) => {
    socketRef.current?.send(JSON.stringify({ type: "message", content: text }));
  };

  return { sendMessageThroughSocket };
}
