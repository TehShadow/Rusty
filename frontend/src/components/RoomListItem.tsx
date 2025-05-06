// components/chat/RoomListItem.tsx
export default function RoomListItem({ room, active, onClick }: {
    room: { id: string, name: string | null };
    active: boolean;
    onClick: () => void;
  }) {
    return (
      <li
        onClick={onClick}
        className={`cursor-pointer p-3 rounded-lg transition ${
          active ? 'bg-blue-500 text-white' : 'bg-gray-100 hover:bg-gray-200'
        }`}
      >
        {room.name || 'Unnamed Room'}
      </li>
    );
  }