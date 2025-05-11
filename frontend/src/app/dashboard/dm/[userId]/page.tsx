'use client'

import { useEffect, useRef, useState } from 'react'
import { useParams } from 'next/navigation'
import api from '@/lib/api'
import { useAuth } from '@/app/components/AuthProvider'

interface DMMessage {
  id?: string
  content: string
  sender_id: string
  created_at: string
}

export default function DirectMessagePage() {
  const { user, token } = useAuth()
  const params = useParams()
  const userId = typeof params.userId === 'string' ? params.userId : ''
  const [messages, setMessages] = useState<DMMessage[]>([])
  const [newMessage, setNewMessage] = useState('')
  const [username, setUsername] = useState('')
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const socketRef = useRef<WebSocket | null>(null)

  // Fetch message history and user info
  useEffect(() => {
    if (!userId) return

    api.get(`/api/dm/${userId}`)
      .then(res => setMessages(res.data))
      .catch(err => console.error('Failed to load DM:', err))

    api.get(`/api/user/${userId}`)
      .then(res => setUsername(res.data.username))
      .catch(err => console.error('Failed to load user:', err))
  }, [userId])

  // WebSocket connection
  useEffect(() => {
    if (typeof window === 'undefined' || !userId || !token) return

    const ws = new WebSocket(`ws://localhost:4000/api/dm/ws/${userId}?token=${token}`)

    ws.onopen = () => console.log('✅ WebSocket connected')

    ws.onmessage = (event) => {
      try {
        const msg: DMMessage = JSON.parse(event.data)
        setMessages(prev => [...prev, msg])
      } catch {
        console.error('❌ Invalid WS message:', event.data)
      }
    }

    ws.onerror = () => console.log('⚠️ WebSocket error')
    ws.onclose = () => console.log('❌ WebSocket disconnected')

    socketRef.current = ws

    return () => {
      ws.close()
      console.log('🧹 WebSocket cleaned up')
    }
  }, [userId, token])

  // Auto-scroll
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  // Send message
  const handleSend = async () => {
    if (!newMessage.trim()) return
    try {
      // await api.post(`/api/dm/${userId}`, { content: newMessage })
      socketRef.current?.send(newMessage)
      setNewMessage('')
    } catch (err) {
      console.error('Failed to send DM:', err)
    }
  }

  if (!user || !userId) return null

  return (
    <div className="flex flex-col h-screen max-h-screen p-4">
      <h1 className="text-xl font-bold mb-4">DM with: {username}</h1>

      <div className="flex-1 overflow-y-auto space-y-2 bg-gray-800 p-4 rounded">
        {messages.map((msg, i) => (
          <div
            key={msg.id || i}
            className={`p-2 rounded max-w-[75%] ${
              msg.sender_id === user.sub
                ? 'bg-blue-600 text-white self-end ml-auto'
                : 'bg-gray-700'
            }`}
          >
            <div className="text-sm text-gray-300">
              {msg.sender_id === user.sub ? 'You' : username}
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
