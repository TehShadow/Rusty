const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:4000';

const getToken = () => localStorage.getItem('token');

const fetchWithAuth = async (url: string, options: RequestInit = {}) => {
  const token = getToken();
  const headers = {
    ...options.headers,
    'Content-Type': 'application/json',
    Authorization: token ? `Bearer ${token}` : '',
  };

  const res = await fetch(`${API_URL}${url}`, {
    ...options,
    headers,
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Request failed with status ${res.status}`);
  }

  // ✅ Don't parse JSON on 204 No Content
  if (res.status === 204) return;

  const text = await res.text();
  return text ? JSON.parse(text) : {};
};

// AUTH
export const loginUser = async (data: { username: string; password: string }) =>
  fetchWithAuth('/login', {
    method: 'POST',
    body: JSON.stringify(data),
    headers: { 'Content-Type': 'application/json' },
  });

export const registerUser = async (data: { username: string; password: string }) =>
  fetchWithAuth('/register', {
    method: 'POST',
    body: JSON.stringify(data),
    headers: { 'Content-Type': 'application/json' },
  });

// USERS
export const fetchUsers = async () => fetchWithAuth('/users');

// FRIENDSHIPS
export const fetchFriends = async () => fetchWithAuth('/friends');

export const addFriend = async (friend_id: string) =>
  fetchWithAuth('/friends', {
    method: 'POST',
    body: JSON.stringify({ friend_id }),
  });

// ROOMS
export const createRoom = async (data: {
  name?: string;
  is_group: boolean;
  member_ids: string[];
}) =>
  fetchWithAuth('/rooms', {
    method: 'POST',
    body: JSON.stringify(data),
  });

export const fetchRooms = async () => fetchWithAuth('/rooms');

export const fetchRoomMembers = async (roomId: string) =>
  fetchWithAuth(`/rooms/${roomId}/members`);

export const addRoomMember = async (roomId: string, userId: string) =>
  fetchWithAuth(`/rooms/${roomId}/members`, {
    method: 'POST',
    body: JSON.stringify(userId),
  });

// MESSAGES
export const fetchMessages = async (roomId: string) =>
  fetchWithAuth(`/rooms/${roomId}/messages`);

export const sendMessage = async (roomId: string, content: string) => 
  fetchWithAuth(`/rooms/${roomId}/messages`, {
    method: 'POST',
    body: JSON.stringify({ content }),
  }
);
