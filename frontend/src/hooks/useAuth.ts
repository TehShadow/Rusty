'use client';

import { useState, useEffect, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import { loginUser, registerUser } from '@/utils/api';

export interface AuthState {
  token: string | null;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string) => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
  loading: boolean;
}

export const useAuth = (): AuthState => {
  const [token, setToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    const storedToken = localStorage.getItem('token');
    if (storedToken) setToken(storedToken);
    setLoading(false);
  }, []);

  const login = useCallback(async (username: string, password: string) => {
    const response = await loginUser({ username, password });
    localStorage.setItem('token', response.token);
    setToken(response.token);
    router.push('/chat');
  }, [router]);

  const register = useCallback(async (username: string, password: string) => {
    const response = await registerUser({ username, password });
    localStorage.setItem('token', response.token);
    setToken(response.token);
    router.push('/chat');
  }, [router]);

  const logout = useCallback(() => {
    localStorage.removeItem('token');
    setToken(null);
    router.push('/login');
  }, [router]);

  return {
    token,
    login,
    register,
    logout,
    isAuthenticated: !!token,
    loading,
  };
};
