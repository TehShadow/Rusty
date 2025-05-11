'use client'
import { useEffect, useRef, useState } from 'react'

export function useWebSocket(roomId: string, token: string | null) {
  const [messages, setMessages] = useState<string[]>([])
  const wsRef = useRef<WebSocket | null>(null)

  useEffect(() => {
    if (!token || !roomId) return

    const socket = new WebSocket(`ws://localhost:4000/api/ws/${roomId}`)

    socket.onopen = () => {
      console.log('✅ WebSocket connected')
      socket.send(JSON.stringify({ type: 'auth', token }))
    }

    socket.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data])
    }

    socket.onclose = () => {
      console.log('❌ WebSocket disconnected')
    }

    wsRef.current = socket

    return () => {
      socket.close()
    }
  }, [roomId, token])

  const sendMessage = (text: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(text)
    }
  }

  return { messages, sendMessage }
}
