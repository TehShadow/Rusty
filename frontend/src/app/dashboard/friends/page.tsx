'use client'
import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import api from '@/lib/api'

interface Friend {
  id: string
  username: string
}

interface PendingRequest {
  id: string
  username: string
}

export default function FriendsPage() {
  const router = useRouter()

  const [friends, setFriends] = useState<Friend[]>([])
  const [pending, setPending] = useState<PendingRequest[]>([])
  const [error, setError] = useState('')
  const [searchId, setSearchId] = useState('')
  const [success, setSuccess] = useState('')

  useEffect(() => {
    const fetchFriends = async () => {
      try {
        const res = await api.get('/api/relationships/friends')
        setFriends(res.data)
      } catch {
        setError('Could not fetch friends.')
      }
    }

    const fetchPending = async () => {
      try {
        const res = await api.get('/api/relationships/pending')
        console.log(res.data)
        setPending(res.data)
      } catch {
        setError('Could not fetch pending requests.')
      }
    }

    fetchFriends()
    fetchPending()
  }, [])

  const handleSendRequest = async () => {
    try {
      await api.post(`/api/relationships/${searchId}`)
      setSuccess('Friend request sent.')
      setError('')
    } catch {
      setError('Failed to send request.')
      setSuccess('')
    }
  }

  const handleRemove = async (id: string) => {
    try {
      await api.delete(`/api/relationships/${id}`)
      setFriends((prev) => prev.filter((f) => f.id !== id))
    } catch {
      setError('Failed to remove friend.')
    }
  }

  const handleAccept = async (id: string) => {
    try {
      await api.post(`/api/relationships/${id}/accept`)
      setPending((prev) => prev.filter((r) => r.id !== id))
    } catch {
      setError('Failed to accept request.')
    }
  }

  const handleDecline = async (id: string) => {
    try {
      await api.delete(`/api/relationships/${id}`)
      setPending((prev) => prev.filter((r) => r.id !== id))
    } catch {
      setError('Failed to decline request.')
    }
  }

  return (
    <div className="max-w-2xl mx-auto mt-8 space-y-6 text-white">
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
        <h2 className="font-semibold mb-2">Pending Friend Requests</h2>
        {pending.length === 0 ? (
          <p className="text-gray-400">No pending requests.</p>
        ) : (
          <ul className="space-y-2">
            {pending.map((req, i) => (
              <li key={i} className="bg-gray-700 p-3 rounded flex justify-between items-center">
                <div>
                  <strong>{req.username}</strong>
                  <p className="text-sm text-gray-400">{req.id}</p>
                </div>
                <div className="space-x-2">
                  <button
                    onClick={() => handleAccept(req.id)}
                    className="bg-green-600 px-2 py-1 rounded hover:bg-green-500"
                  >
                    Accept
                  </button>
                  <button
                    onClick={() => handleDecline(req.id)}
                    className="bg-red-600 px-2 py-1 rounded hover:bg-red-500"
                  >
                    Decline
                  </button>
                </div>
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
