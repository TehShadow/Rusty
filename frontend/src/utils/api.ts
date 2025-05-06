const API_BASE = "http://localhost:3001";

export async function loginUser(username: string, password: string): Promise<string | null> {
  try {
    const res = await fetch(`${API_BASE}/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) return null;
    const data = await res.json();
    return data.token;
  } catch (err) {
    console.error("Login error", err);
    return null;
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