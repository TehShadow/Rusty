// components/chat/MessageList.tsx
export default function MessageList({ messages }: {
    messages: { sender_id: string, content: string }[];
  }) {
    return (
      <div className="flex-1 overflow-y-auto space-y-3 pr-2">
        {messages.map((msg, i) => (
          <div key={i} className="flex flex-col border-b pb-2">
            <span className="text-sm text-gray-600 font-semibold">
              {msg.sender_id}
            </span>
            <span className="text-base text-gray-800">{msg.content}</span>
          </div>
        ))}
      </div>
    );
  }