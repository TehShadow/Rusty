// components/chat/RoomList.tsx
import RoomListItem from './RoomListItem';

export default function RoomList({ rooms, selectedRoomId, onSelect }: {
  rooms: any[];
  selectedRoomId: string | null;
  onSelect: (id: string) => void;
}) {
  return (
    <ul className="space-y-2">
      {rooms.map(room => (
        <RoomListItem
          key={room.id}
          room={room}
          active={room.id === selectedRoomId}
          onClick={() => onSelect(room.id)}
        />
      ))}
    </ul>
  );
}