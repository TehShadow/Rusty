'use client';

import { useEffect, useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import { useUser } from '@/hooks/useUser';
import { useRouter } from 'next/navigation';
import {
  fetchRooms,
  fetchMessages,
  fetchFriends,
  addFriend,
  createRoom,
} from '@/utils/api';
import { chatSocket } from '@/utils/socket';

import ChatLayout from '@/components/ChatLayout';
import RoomList from '@/components/RoomList';
import MessageList from '@/components/MessageList';
import MessageInput from '@/components/MessageInput';
import FriendList from '@/components/FriendList';
import AddFriendForm from '@/components/AddFriendForm';
import CreateRoomForm from '@/components/CreateRoomForm';

export default function ChatPage() {
  const { token, logout, isAuthenticated } = useAuth();
  const { user, loading, refetch } = useUser();
  const router = useRouter();

  const [rooms, setRooms] = useState([]);
  const [friends, setFriends] = useState([]);
  const [messages, setMessages] = useState([]);
  const [selectedRoomId, setSelectedRoomId] = useState(null);
  const [messageInput, setMessageInput] = useState('');
  const [activeTab, setActiveTab] = useState<'rooms' | 'friends'>('rooms');

  useEffect(() => {
    if (!loading && !isAuthenticated) router.push('/login');
    else if (isAuthenticated) {
      refetch();
      loadRooms();
      loadFriends();
    }
  }, [loading, isAuthenticated]);

  useEffect(() => {
    if (!selectedRoomId || !token) return;

    setMessages([]);
    chatSocket.connect(selectedRoomId, token);
    chatSocket.onMessage((msg) => setMessages((prev) => [...prev, msg]));
    loadMessages(selectedRoomId);

    return () => chatSocket.close();
  }, [selectedRoomId, token]);

  const loadRooms = async () => setRooms(await fetchRooms());
  const loadMessages = async (id) => setMessages(await fetchMessages(id));
  const loadFriends = async () => setFriends(await fetchFriends());

  const handleSendMessage = () => {
    if (!messageInput.trim()) return;
    chatSocket.send(messageInput.trim());
    setMessageInput('');
  };

  const handleAddFriend = async (id: string) => {
    await addFriend(id);
    await loadFriends();
  };

  const handleCreateRoom = async (name: string, memberIds: string[]) => {
    const response = await createRoom({ name, is_group: true, member_ids: memberIds });
    loadRooms();
    setSelectedRoomId(response.id);
    setActiveTab('rooms');
  };

  const handleStartChatWithFriend = async (friendId: string) => {
    const response = await createRoom({ is_group: false, member_ids: [friendId] });
    loadRooms();
    setSelectedRoomId(response.id);
    setActiveTab('rooms');
  };

  return (
    <ChatLayout>
      <div className="col-span-3 bg-white rounded-xl shadow p-4 overflow-y-auto">
        <div className="flex gap-4 mb-4">
          <button
            onClick={() => setActiveTab('rooms')}
            className={`text-sm font-semibold px-3 py-1 rounded ${
              activeTab === 'rooms' ? 'bg-blue-500 text-white' : 'bg-gray-200'
            }`}
          >
            Rooms
          </button>
          <button
            onClick={() => setActiveTab('friends')}
            className={`text-sm font-semibold px-3 py-1 rounded ${
              activeTab === 'friends' ? 'bg-blue-500 text-white' : 'bg-gray-200'
            }`}
          >
            Friends
          </button>
        </div>

        {activeTab === 'rooms' ? (
          <>
            <CreateRoomForm friends={friends} onCreate={handleCreateRoom} />
            <RoomList
              rooms={rooms}
              selectedRoomId={selectedRoomId}
              onSelect={setSelectedRoomId}
            />
          </>
        ) : (
          <>
            <AddFriendForm onAdd={handleAddFriend} />
            <FriendList friends={friends} onChat={handleStartChatWithFriend} />
          </>
        )}
      </div>

      <div className="col-span-9 flex flex-col bg-white rounded-xl shadow p-4">
        <MessageList messages={messages} />
        <MessageInput
          value={messageInput}
          onChange={setMessageInput}
          onSend={handleSendMessage}
        />
      </div>
    </ChatLayout>
  );
}