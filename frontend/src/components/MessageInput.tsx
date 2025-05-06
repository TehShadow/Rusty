// components/chat/MessageInput.tsx
export default function MessageInput({ value, onChange, onSend }: {
    value: string;
    onChange: (val: string) => void;
    onSend: () => void;
  }) {
    return (
      <div className="mt-4 flex items-center">
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="flex-1 border rounded-lg px-4 py-2 text-sm shadow-sm"
          placeholder="Type a message..."
        />
        <button
          onClick={onSend}
          disabled={!value.trim()}
          className="ml-3 bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg text-sm"
        >
          Send
        </button>
      </div>
    );
  }