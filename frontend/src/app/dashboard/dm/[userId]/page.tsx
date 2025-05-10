'use client'
import { useEffect, useRef, useState } from 'react'
import { useParams } from 'next/navigation'
import api from '@/lib/api'
import { useAuth } from '@/app/components/AuthProvider'

interface DMMessage {
  id: string
  content: string
  author_id: string
  created_at: string
}

export default function DirectMessagePage() {
  const { user } = useAuth()
  const { userId } = useParams()
  const [messages, setMessages] = useState<DMMessage[]>([])
  const [newMessage, setNewMessage] = useState('')
  const [username,setUsername] = useState('')
  const messagesEndRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const fetchMessages = async () => {
      try {
        const res = await api.get(`/api/user/dm/${userId}`)
        setMessages(res.data)
      } catch (err) {
        console.error('Failed to load DM:', err)
      }
    }

    const fetchuser = async () => {
        try{
            const res = await api.get(`/api/user/${userId}`)
            setUsername(res.data.username)
        }catch(err){
            console.error('Failed to load user',err)
        }
    }


    if (userId) fetchMessages()
    fetchuser()
  }, [userId])

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  const handleSend = async () => {
    if (!newMessage.trim()) return
    try {
      const res = await api.post(`/api/dm/${userId}`, { content: newMessage })
      setMessages(prev => [...prev, res.data])
      setNewMessage('')
    } catch (err) {
      console.error('Failed to send DM:', err)
    }
  }

  return (
    <div className="flex flex-col h-screen max-h-screen p-4">
      <h1 className="text-xl font-bold mb-4">DM with: {username}</h1>

      <div className="flex-1 overflow-y-auto space-y-2 bg-gray-800 p-4 rounded">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`p-2 rounded ${
              msg.author_id === user?.id ? 'bg-blue-600 text-white self-end' : 'bg-gray-700'
            }`}
          >
            <div className="text-sm text-gray-300">
              {msg.author_id === user?.id ? 'You' : msg.author_id}
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
