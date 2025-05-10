import './globals.css'
import { AuthProvider } from '@/app/components/AuthProvider'

export const metadata = {
  title: 'Rusty',
  description: 'Rusty Rust Backend',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="bg-gray-900 text-white">
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  )
}