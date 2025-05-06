// components/chat/FriendList.tsx
export default function FriendList({ friends, onChat }: {
    friends: { id: string, username: string }[];
    onChat: (id: string) => void;
  }) {
    return (
      <ul className="space-y-2">
        {friends.map(friend => (
          <li
            key={friend.id}
            className="flex justify-between items-center bg-gray-100 p-2 rounded-lg text-sm"
          >
            {friend.username}
            <button
              onClick={() => onChat(friend.id)}
              className="text-blue-500 hover:underline text-xs"
            >
              💬 Chat
            </button>
          </li>
        ))}
      </ul>
    );
  }