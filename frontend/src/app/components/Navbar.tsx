'use client'
import { useState, useEffect, useRef } from 'react'
import Link from 'next/link'
import { useAuth } from './AuthProvider'
import { usePathname, useRouter } from 'next/navigation'
import { Menu, X } from 'lucide-react'

export default function Navbar() {
  const { user, logout } = useAuth()
  const pathname = usePathname()
  const router = useRouter()
  const [open, setOpen] = useState(false)
  const menuRef = useRef<HTMLDivElement | null>(null)

  const handleLogout = () => {
    logout()
    router.push('/login')
  }

  const isActive = (path: string) =>
    pathname.startsWith(path) ? 'text-blue-400' : 'text-white'

  // ✅ Close menu on click outside (mobile only)
  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpen(false)
      }
    }
    if (open) document.addEventListener('mousedown', handler)
    return () => document.removeEventListener('mousedown', handler)
  }, [open])

  if (!user) return null

  return (
    <>
      {/* Mobile Top Bar */}
      <div className="md:hidden bg-gray-900 px-4 py-3 flex justify-between items-center shadow">
        <span className="text-blue-300 font-bold text-xl">R</span>
        <button onClick={() => setOpen(true)} className="text-white">
          <Menu size={24} />
        </button>
      </div>

      {/* Mobile Slide-In Menu */}
      {open && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-40">
          <div
            ref={menuRef}
            className="fixed top-0 left-0 h-full w-64 bg-gray-800 p-4 z-50 shadow-lg transform translate-x-0 transition-transform"
          >
            <div className="flex justify-between items-center mb-6">
              <span className="text-blue-300 text-lg font-bold">Rusty</span>
              <button onClick={() => setOpen(false)} className="text-white">
                <X size={20} />
              </button>
            </div>
            <ul className="space-y-4">
              <li>
                <Link href="/dashboard" onClick={() => setOpen(false)} className={isActive('/dashboard')}>
                  🏠 Dashboard
                </Link>
              </li>
              <li>
                <Link href="/dashboard/rooms" onClick={() => setOpen(false)} className={isActive('/dashboard/rooms')}>
                  🏘️ Rooms
                </Link>
              </li>
              <li>
                <Link href="/dashboard/dm" onClick={() => setOpen(false)} className={isActive('/dashboard/dm')}>
                  💬 Direct Messages
                </Link>
              </li>
              <li>
                <Link href="/dashboard/friends" onClick={() => setOpen(false)} className={isActive('/dashboard/friends')}>
                  👥 Friends
                </Link>
              </li>
            </ul>
            <div className="mt-10">
              <p className="text-sm text-gray-400 mb-2">
                Logged in as <strong>{user.username}</strong>
              </p>
              <button
                onClick={handleLogout}
                className="w-full px-3 py-2 bg-red-600 hover:bg-red-500 rounded text-white text-sm"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Desktop Sidebar */}
      <div className="hidden md:flex flex-col w-60 h-screen bg-gray-800 p-4 shadow-xl fixed top-0 left-0 z-30">
        <h2 className="text-xl font-bold mb-6 text-center text-blue-300">Rustcord</h2>
        <ul className="space-y-4">
          <li>
            <Link href="/dashboard" className={isActive('/dashboard')}>🏠 Dashboard</Link>
          </li>
          <li>
            <Link href="/dashboard/rooms" className={isActive('/dashboard/rooms')}>🏘️ Rooms</Link>
          </li>
          <li>
            <Link href="/dashboard/friends" className={isActive('/dashboard/friends')}>👥 Friends</Link>
          </li>
        </ul>
        <div className="mt-auto">
          <p className="text-sm text-gray-400 mb-2">Logged in as <strong>{user.username}</strong></p>
          <button
            onClick={handleLogout}
            className="w-full px-3 py-2 bg-red-600 hover:bg-red-500 rounded text-white text-sm"
          >
            Logout
          </button>
        </div>
      </div>
    </>
  )
}
