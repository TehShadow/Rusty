import Navbar from '@/app/components/Navbar'

export default function DashboardLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex">
      <Navbar />
      <main className="flex-1 md:ml-60 p-4">{children}</main>
    </div>
  )
}
