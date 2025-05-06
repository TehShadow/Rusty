// components/chat/ChatLayout.tsx
import { ReactNode } from 'react';

export default function ChatLayout({ children }: { children: ReactNode }) {
  return (
    <div className="p-6 h-screen bg-gray-50">
      <div className="grid grid-cols-12 gap-6 h-full">
        {children}
      </div>
    </div>
  );
}