'use client'
import { useAuth } from '@/app/components/AuthProvider'
import { useRouter } from 'next/navigation'
import { useEffect } from 'react'

export default function DashboardPage() {
  const { user, token } = useAuth()
  const router = useRouter()

  useEffect(() => {
    if (!token) router.push('/login')
  }, [token])

  if (!token) return null

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">Welcome, {user?.username}</h1>
      <p className='text-1xl font-bold'>{user?.username} : {user?.sub}</p>
    </div>
  )
}
