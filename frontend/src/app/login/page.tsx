'use client'
import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/app/components/AuthProvider'
import api from '@/lib/api'

export default function LoginPage() {
  const { token, login } = useAuth()
  const router = useRouter()

  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')

  // ✅ Redirect to dashboard if already logged in
  useEffect(() => {
    if (token) router.push('/dashboard')
  }, [token])

  const handleLogin = async () => {
    try {
      const res = await api.post('/api/login', { username, password })
      login(res.data.token)
      router.push('/dashboard')
    } catch (err) {
      setError('Login failed. Check your credentials.')
    }
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center gap-4">
      <h1 className="text-3xl font-bold">Login</h1>
      {error && <p className="text-red-500">{error}</p>}
      <input
        className="p-2 bg-gray-800 border border-gray-700 rounded w-72"
        placeholder="Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <input
        type="password"
        className="p-2 bg-gray-800 border border-gray-700 rounded w-72"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button
        onClick={handleLogin}
        className="px-4 py-2 mt-2 bg-green-600 hover:bg-green-500 rounded text-white"
      >
        Log In
      </button>
      <p className="text-sm">
        Don't have an account?{' '}
        <a href="/register" className="underline text-blue-400">
          Register
        </a>
      </p>
    </div>
  )
}
