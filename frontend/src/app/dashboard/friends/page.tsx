'use client'
import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import api from '@/lib/api'
import { useAuth } from '@/app/components/AuthProvider'

interface Friend {
  id: string
  username: string
}

export default function FriendsPage() {
  const { user } = useAuth()
  const router = useRouter()

  const [friends, setFriends] = useState<Friend[]>([])
  const [error, setError] = useState('')
  const [searchId, setSearchId] = useState('')
  const [success, setSuccess] = useState('')

  useEffect(() => {
    const fetchFriends = async () => {
      try {
        const res = await api.get('/api/relationships/friends')
        setFriends(res.data)
      } catch (err) {
        setError('Could not fetch friends.')
      }
    }

    fetchFriends()
  }, [])

  const handleSendRequest = async () => {
    try {
      await api.post(`/api/relationships/${searchId}`)
      setSuccess('Friend request sent.')
      setError('')
    } catch (err) {
      setError('Failed to send request.')
      setSuccess('')
    }
  }

  const handleRemove = async (id: string) => {
    try {
      await api.delete(`/api/relationships/${id}`)
      setFriends((prev) => prev.filter((f) => f.id !== id))
    } catch (err) {
      setError('Failed to remove friend.')
    }
  }

  return (
    <div className="max-w-2xl mx-auto mt-8 space-y-6">
      <h1 className="text-2xl font-bold">👥 Friends</h1>

      <section>
        <h2 className="font-semibold mb-2">Your Friends</h2>
        {friends.length === 0 ? (
          <p className="text-gray-400">No friends yet.</p>
        ) : (
          <ul className="space-y-2">
            {friends.map((f) => (
              <li
                key={f.id}
                className="bg-gray-800 p-3 rounded flex justify-between items-center hover:bg-gray-700 cursor-pointer"
                onClick={() => router.push(`/dashboard/dm/${f.id}`)}
              >
                <div>
                  <strong>{f.username}</strong>
                  <p className="text-sm text-gray-400">{f.id}</p>
                </div>
                <button
                  onClick={(e) => {
                    e.stopPropagation()
                    handleRemove(f.id)
                  }}
                  className="text-sm px-3 py-1 bg-red-600 hover:bg-red-500 rounded"
                >
                  Remove
                </button>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="mt-6">
        <h2 className="font-semibold mb-2">Add a Friend</h2>
        <input
          type="text"
          value={searchId}
          onChange={(e) => setSearchId(e.target.value)}
          placeholder="User ID"
          className="p-2 w-full bg-gray-800 border border-gray-600 rounded mb-2"
        />
        <button
          onClick={handleSendRequest}
          className="w-full bg-blue-600 hover:bg-blue-500 text-white py-2 rounded"
        >
          Send Friend Request
        </button>
        {error && <p className="text-red-500 mt-2">{error}</p>}
        {success && <p className="text-green-500 mt-2">{success}</p>}
      </section>
    </div>
  )
}
