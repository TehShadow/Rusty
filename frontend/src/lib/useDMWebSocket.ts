// hooks/useDMWebSocket.ts
import { useEffect, useRef } from "react";

export function useDMWebSocket({
  token,
  otherUserId,
  onMessage,
}: {
  token: string;
  otherUserId: string;
  onMessage: (msg: any) => void;
}) {
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(
      `ws://localhost:4000/api/dm/ws/${otherUserId}?token=${token}`
    );

    ws.onmessage = (event) => {
      try {
        const parsed = JSON.parse(event.data);
        onMessage(parsed);
      } catch (_) {
        console.error("Invalid WS message:", event.data);
      }
    };

    wsRef.current = ws;

    return () => {
      ws.close();
    };
  }, [token, otherUserId]);

  const sendMessage = (text: string) => {
    wsRef.current?.send(text);
  };

  return { sendMessage };
}
