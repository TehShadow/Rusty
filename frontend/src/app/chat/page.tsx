// app/chat/page.tsx
"use client";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { fetchMe, fetchRooms, fetchMessages } from "@/utils/api";
import useChatSocket from "@/hooks/useChatSocket";

export default function ChatPage() {
  const router = useRouter();
  const [me, setMe] = useState<any>(null);
  const [rooms, setRooms] = useState<any[]>([]);
  const [selectedRoom, setSelectedRoom] = useState<any | null>(null);
  const [messages, setMessages] = useState<any[]>([]);
  const [input, setInput] = useState("");

  const { sendMessageThroughSocket } = useChatSocket(selectedRoom, setMessages);

  useEffect(() => {
    const token = localStorage.getItem("jwt");
    if (!token) {
      router.push("/login");
      return;
    }
    fetchMe(token).then(setMe);
    fetchRooms(token).then(setRooms);
  }, [router]);

  useEffect(() => {
    const token = localStorage.getItem("jwt")!;
    if (selectedRoom) {
      fetchMessages(selectedRoom.id, token).then(setMessages);
    }
  }, [selectedRoom]);

  const sendMessage = () => {
    if (input.trim() === "") return;
    sendMessageThroughSocket(input);
    setMessages((prev) => [
      ...prev,
      { sender_id: me?.id, content: input },
    ]);
    setInput("");
  };

  return (
    <div className="min-h-screen bg-gradient-to-r from-sky-50 to-indigo-50">
      <div className="max-w-6xl mx-auto mt-10 bg-white rounded-xl shadow-xl p-8">
        <h1 className="text-4xl font-bold mb-8 text-center text-gray-800">Welcome, {me?.username || "..."}</h1>
        <div className="grid grid-cols-12 gap-8">
          <div className="col-span-4 bg-white border rounded-lg shadow-sm p-4 overflow-y-auto max-h-[600px]">
            <h2 className="text-xl font-semibold mb-4 text-gray-700">Chat Rooms</h2>
            <div className="space-y-2">
              {rooms.map((room) => (
                <div
                  key={room.id}
                  onClick={() => setSelectedRoom(room)}
                  className={`p-3 rounded-lg cursor-pointer transition-all duration-200 ${
                    selectedRoom?.id === room.id ? "bg-indigo-100 border border-indigo-400 text-indigo-700 font-semibold" : "hover:bg-gray-100"
                  }`}
                >
                  {room.name || "Private Chat"}
                </div>
              ))}
            </div>
          </div>

          <div className="col-span-8 bg-gray-50 rounded-lg shadow-inner p-6 flex flex-col">
            {selectedRoom ? (
              <>
                <div className="border-b pb-4 mb-4">
                  <h2 className="text-2xl font-semibold text-gray-800">
                    {selectedRoom.name || "Private Chat"}
                  </h2>
                </div>
                <div className="flex-1 overflow-y-auto space-y-4 px-1">
                  {messages.map((msg, i) => (
                    <div
                      key={i}
                      className={`inline-block max-w-[70%] px-4 py-3 rounded-3xl text-sm ${
                        msg.sender_id === me?.id
                          ? "ml-auto bg-indigo-600 text-white"
                          : "bg-white text-gray-900 border border-gray-200"
                      }`}
                    >
                      {msg.content}
                    </div>
                  ))}
                </div>
                <div className="mt-6 flex gap-4 border-t pt-4">
                  <input
                    className="flex-1 px-4 py-3 border border-gray-300 rounded-full focus:outline-none focus:ring-2 focus:ring-indigo-500"
                    placeholder="Type your message..."
                    value={input}
                    onChange={(e) => setInput(e.target.value)}
                  />
                  <button
                    className="px-6 py-3 bg-indigo-600 text-white rounded-full hover:bg-indigo-700 transition"
                    onClick={sendMessage}
                  >
                    Send
                  </button>
                </div>
              </>
            ) : (
              <div className="text-center text-gray-500 flex-1 flex items-center justify-center">
                Select a room to start chatting
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
