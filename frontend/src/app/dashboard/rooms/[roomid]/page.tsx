'use client'

import { useEffect, useRef, useState } from 'react'
import { useParams } from 'next/navigation'
import api from '@/lib/api'
import { useAuth } from '@/app/components/AuthProvider'

interface RoomInfo {
  id: string
  name: string
  owner_id: string
  created_at: string
}

interface WSMessage {
  author_id: string
  content: string
  created_at: string
}

export default function RoomChatPage() {
  const params = useParams<{ roomid: string }>()
  const roomid = typeof params.roomid === 'string' ? params.roomid : ''
  const { user, token } = useAuth()

  const [room, setRoom] = useState<RoomInfo | null>(null)
  const [messages, setMessages] = useState<WSMessage[]>([])
  const [members, setMembers] = useState<Record<string, string>>({})
  const [newMessage, setNewMessage] = useState('')
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const socketRef = useRef<WebSocket | null>(null)

  // ✅ Fetch room info
  useEffect(() => {
    if (!roomid) return
    api.get(`/api/rooms/${roomid}`)
      .then(res => setRoom(res.data))
      .catch(err => console.error('Failed to fetch room info:', err))
  }, [roomid])

  // ✅ Fetch room members
  useEffect(() => {
    if (!roomid) return
    api.get(`/api/rooms/${roomid}/members`)
      .then(res => {
        const map: Record<string, string> = {}
        res.data.forEach((m: { id: string; username: string }) => {
          map[m.id] = m.username
        })
        setMembers(map)
      })
      .catch(err => console.error('Failed to load members:', err))
  }, [roomid])

  // ✅ Fetch message history
  useEffect(() => {
    if (!roomid) return
    api.get(`/api/rooms/${roomid}/messages`)
      .then(res => setMessages(res.data))
      .catch(err => console.error('Failed to load messages:', err))
  }, [roomid])

  // ✅ Auto-scroll
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  // ✅ WebSocket connection
  useEffect(() => {
    if (typeof window === 'undefined' || !roomid || !token) return

    const ws = new WebSocket(`ws://localhost:4000/api/ws/${roomid}?token=${token}`)

    ws.onopen = () => console.log('✅ WebSocket connected')

    ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data) as WSMessage
        setMessages((prev) => [...prev, msg])
      } catch (err) {
        console.error('Invalid WS message:', event.data)
      }
    }

    ws.onclose = () => console.log('❌ WebSocket disconnected')
    ws.onerror = (err) => console.error('WebSocket error:', err)

    socketRef.current = ws

    return () => {
      ws.close()
      console.log('🧹 WebSocket cleaned up')
    }
  }, [roomid, token])

  // ✅ Send message
  const handleSend = () => {
    if (!newMessage.trim()) return
    if (socketRef.current?.readyState === WebSocket.OPEN) {
      socketRef.current.send(newMessage)
      setNewMessage('')
    }
  }

  if (!roomid || !room) {
    return (
      <div className="p-6 text-gray-300 text-center">
        Loading room...
      </div>
    )
  }

  return (
    <div className="flex flex-col h-screen max-h-screen p-4">
      <h1 className="text-xl font-bold mb-4">Room: {room.name}</h1>

      <div className="flex-1 overflow-y-auto space-y-2 bg-gray-800 p-4 rounded">
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`p-2 rounded max-w-[75%] ${
              msg.author_id === user?.sub ? 'bg-blue-600 text-white self-end ml-auto' : 'bg-gray-700'
            }`}
          >
            <div className="text-sm text-gray-300">
              {msg.author_id === user?.sub ? 'You' : members[msg.author_id] || msg.author_id}
              <span className="text-xs text-gray-400 ml-2">
                {new Date(msg.created_at).toLocaleTimeString()}
              </span>
            </div>
            <p>{msg.content}</p>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      <div className="mt-4 flex gap-2">
        <input
          type="text"
          className="flex-1 p-2 rounded bg-gray-800 border border-gray-700"
          placeholder="Type a message..."
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSend()}
        />
        <button
          onClick={handleSend}
          className="px-4 py-2 bg-green-600 rounded hover:bg-green-500"
        >
          Send
        </button>
      </div>
    </div>
  )
}
