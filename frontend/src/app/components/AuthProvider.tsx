'use client';

import { createContext, useContext, useEffect, useState } from 'react';
import { jwtDecode } from 'jwt-decode';

// ✅ Define your token structure
interface DecodedToken {
  sub: string;
  username: string;
  session_id: string;
  exp: number;
}

interface AuthContextType {
  user: DecodedToken | null;
  token: string | null;
  login: (token: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [token, setToken] = useState<string | null>(null);
  const [user, setUser] = useState<DecodedToken | null>(null);

  useEffect(() => {
    const stored = localStorage.getItem('token');
    console.log(user)
    if (stored) {
      try {
        const decoded = jwtDecode<DecodedToken>(stored);
        setToken(stored);
        setUser(decoded);
      } catch (err) {
        console.error('Invalid stored JWT:', err);
        localStorage.removeItem('token');
      }
    }
  }, []);

  const login = (jwt: string) => {
    try {
      const decoded = jwtDecode<DecodedToken>(jwt);
      localStorage.setItem('token', jwt);
      setToken(jwt);
      setUser(decoded);
    } catch (err) {
      console.error('Invalid login JWT:', err);
    }
  };

  const logout = () => {
    localStorage.removeItem('token');
    setToken(null);
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, token, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
