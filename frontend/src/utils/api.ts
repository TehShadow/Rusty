const API_BASE = "http://localhost:3001";

export async function loginUser(username: string, password: string): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) return false;
    const data = await res.json();
    localStorage.setItem("jwt", data.token);
    return true;
  } catch (err) {
    console.error("Login error", err);
    return false;
  }
}

export async function registerUser(username: string, password: string): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });
    return res.ok;
  } catch (err) {
    console.error("Register error", err);
    return false;
  }
}

export async function fetchMe(token: string): Promise<any | null> {
  try {
    const res = await fetch(`${API_BASE}/me`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) return null;
    return await res.json();
  } catch (err) {
    console.error("Fetch me error", err);
    return null;
  }
}

export async function fetchRooms(token: string): Promise<any[]> {
  try {
    const res = await fetch(`${API_BASE}/rooms`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) return [];
    return await res.json();
  } catch (err) {
    console.error("Fetch rooms error", err);
    return [];
  }
}

export async function fetchMessages(roomId: string, token: string): Promise<any[] | null> {
  try {
    const res = await fetch(`${API_BASE}/rooms/${roomId}/messages`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) return null;
    return await res.json();
  } catch (err) {
    console.error("Fetch messages error", err);
    return null;
  }
}

export async function sendMessageToRoom(roomId: string, content: string, token: string): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/rooms/${roomId}/messages`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({ content }),
    });
    return res.ok;
  } catch (err) {
    console.error("Send message error", err);
    return false;
  }
}