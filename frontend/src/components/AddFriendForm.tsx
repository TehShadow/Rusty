// components/chat/AddFriendForm.tsx
import { useState } from 'react';

export default function AddFriendForm({ onAdd }: { onAdd: (id: string) => void }) {
  const [input, setInput] = useState('');

  return (
    <div className="mb-4">
      <input
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Friend ID"
        className="w-full border rounded px-2 py-1 text-sm mb-2"
      />
      <button
        onClick={() => {
          if (input.trim()) onAdd(input.trim());
          setInput('');
        }}
        className="bg-green-500 text-white px-3 py-1 rounded text-sm w-full"
      >
        Add Friend
      </button>
    </div>
  );
}
