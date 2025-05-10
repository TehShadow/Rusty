'use client'
import { useState } from 'react'
import { useRouter } from 'next/navigation'
import api from '@/lib/api'

export default function RegisterPage() {
  const router = useRouter()
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')

  const handleRegister = async () => {
    try {
      await api.post('/api/register', { username, password })
      router.push('/login')
    } catch (err: any) {
      setError(err?.response?.data?.message || 'Registration failed')
    }
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-900 text-white gap-4">
      <h1 className="text-3xl font-bold">Create an Account</h1>
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
        onClick={handleRegister}
        className="px-4 py-2 mt-2 bg-green-600 hover:bg-green-500 rounded"
      >
        Register
      </button>
      <p className="text-sm">
        Already have an account? <a href="/login" className="underline text-blue-400">Login</a>
      </p>
    </div>
  )
}
