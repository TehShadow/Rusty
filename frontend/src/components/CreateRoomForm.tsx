// components/chat/CreateRoomForm.tsx
import { useState } from 'react';

export default function CreateRoomForm({
  friends,
  onCreate,
}: {
  friends: { id: string; username: string }[];
  onCreate: (name: string, memberIds: string[]) => void;
}) {
  const [name, setName] = useState('');
  const [selected, setSelected] = useState<string[]>([]);

  return (
    <div className="mb-4">
      <input
        type="text"
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Room name"
        className="w-full border rounded px-2 py-1 text-sm mb-2"
      />
      <div className="text-xs text-gray-500 mb-1">Select friends:</div>
      <div className="flex flex-col gap-1 max-h-32 overflow-y-auto border p-2 rounded">
        {friends.map((f) => (
          <label key={f.id} className="flex gap-2 text-sm">
            <input
              type="checkbox"
              checked={selected.includes(f.id)}
              onChange={() =>
                setSelected((prev) =>
                  prev.includes(f.id)
                    ? prev.filter((id) => id !== f.id)
                    : [...prev, f.id]
                )
              }
            />
            {f.username}
          </label>
        ))}
      </div>
      <button
        onClick={() => {
          if (name.trim()) onCreate(name.trim(), selected);
        }}
        className="mt-2 bg-purple-500 text-white px-3 py-1 rounded text-sm"
      >
        Create Room
      </button>
    </div>
  );
}
