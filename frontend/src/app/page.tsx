'use client'
import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/app/components/AuthProvider'

export default function HomePage() {
  const { token } = useAuth()
  const router = useRouter()

  useEffect(() => {
    if (token) router.push('/dashboard')
    else router.push('/login')
  }, [token])

  return null
}