'use client'
import { useEffect, useState } from 'react'
import api from '@/lib/api'
import { useAuth } from '@/app/components/AuthProvider'
import { useRouter } from 'next/navigation'

interface Room {
  id: string
  name: string
  created_at: string
}

export default function RoomsPage() {
  const { token } = useAuth()
  const router = useRouter()

  const [rooms, setRooms] = useState<Room[]>([])
  const [joinId, setJoinId] = useState('')
  const [newRoomName, setNewRoomName] = useState('')
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  useEffect(() => {
    const fetchRooms = async () => {
      try {
        const res = await api.get('/api/rooms')
        setRooms(res.data)
      } catch (err) {
        setError('Failed to load rooms.')
      }
    }

    if (token) fetchRooms()
  }, [token])

  const handleJoin = async () => {
    try {
      await api.post(`/api/rooms/${joinId}/join`)
      router.push(`/dashboard/rooms/${joinId}`)
    } catch (err) {
      setError('Invalid room ID or already joined.')
    }
  }

  const handleCreateRoom = async () => {
    if (!newRoomName.trim()) return
    try {
      const res = await api.post('/api/rooms', { name: newRoomName })
      setRooms((prev) => [...prev, res.data])
      setNewRoomName('')
      setSuccess('Room created.')
      setError('')
    } catch (err) {
      setError('Failed to create room.')
    }
  }

  const handleCopyButton = async (room_id:string) => {
    try{
      await navigator.clipboard.writeText(room_id);
    }
    catch(err){
      console.log(err)
    }
  }

  return (
    <div className="max-w-2xl mx-auto mt-8 space-y-8">
      <h1 className="text-2xl font-bold">🏘️ Joined Rooms</h1>

      {rooms.length === 0 ? (
        <p>No joined rooms yet.</p>
      ) : (
        <ul className="space-y-2">
          {rooms.map((room) => (
            <li
              key={room.id}
              className="p-3 bg-gray-800 rounded flex justify-between items-center hover:bg-gray-700 cursor-pointer"
              onClick={() => router.push(`/dashboard/rooms/${room.id}`)}
            >
              <div>
                <strong>{room.name}</strong>
                <span className="text-sm text-gray-400 ml-2">({room.id})</span>
              </div>
              <button
                  onClick={(e)=>{
                    e.stopPropagation()
                    handleCopyButton(room.id)
                  }}
                  className="text-sm px-3 py-1 bg-blue-600 hover:bg-blue-500 rounded"
              >
              Copy ID
            </button>
            </li>
          ))}
        </ul>
      )}

      <div>
        <h2 className="text-lg font-semibold mb-2">Join a Room</h2>
        <input
          type="text"
          value={joinId}
          onChange={(e) => setJoinId(e.target.value)}
          placeholder="Room ID"
          className="p-2 w-full bg-gray-800 border border-gray-600 rounded mb-2"
        />
        <button
          onClick={handleJoin}
          className="w-full bg-blue-600 hover:bg-blue-500 text-white py-2 rounded"
        >
          Join Room
        </button>
      </div>

      <div>
        <h2 className="text-lg font-semibold mb-2">Create a Room</h2>
        <input
          type="text"
          value={newRoomName}
          onChange={(e) => setNewRoomName(e.target.value)}
          placeholder="Room Name"
          className="p-2 w-full bg-gray-800 border border-gray-600 rounded mb-2"
        />
        <button
          onClick={handleCreateRoom}
          className="w-full bg-green-600 hover:bg-green-500 text-white py-2 rounded"
        >
          Create Room
        </button>
        {error && <p className="text-red-500 mt-2">{error}</p>}
        {success && <p className="text-green-500 mt-2">{success}</p>}
      </div>
    </div>
  )
}
